//! Database Module
//!
//! Uses SQLite to store command-to-action mappings

use crate::error::{FrameworkError, FrameworkResult};
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};
use parking_lot::Mutex;

/// Command mapping record
#[derive(Debug, Clone)]
pub struct CommandMapping {
    /// Command name (e.g., "version", "help")
    pub command: String,
    /// Corresponding action (e.g., "show_version", "show_help")
    pub action: String,
    /// Description
    pub description: String,
    /// Whether enabled
    pub enabled: bool,
}

/// Database manager
pub struct Database {
    conn: Mutex<Connection>,
    db_path: PathBuf,
}

impl Database {
    /// Create a new database instance
    pub fn new(db_path: impl AsRef<Path>) -> FrameworkResult<Self> {
        let db_path = db_path.as_ref().to_path_buf();
        let conn = Connection::open(&db_path)
            .map_err(|e| FrameworkError::InitError(format!(
                "Failed to open database: {}", e
            )))?;

        let db = Self {
            conn: Mutex::new(conn),
            db_path,
        };

        db.init_tables()?;
        Ok(db)
    }

    /// Initialize database tables
    fn init_tables(&self) -> FrameworkResult<()> {
        let conn = self.conn.lock();
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS command_mappings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                command TEXT NOT NULL UNIQUE,
                action TEXT NOT NULL,
                description TEXT,
                enabled INTEGER NOT NULL DEFAULT 1,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).map_err(|e| FrameworkError::InitError(format!(
            "Failed to create table: {}", e
        )))?;

        // Insert default command mappings
        let default_mappings = vec![
            ("version", "show_version", "Display version information"),
            ("help", "show_help", "Display help information"),
            ("sysinfo", "call_sysinfo_dll", "Display system information"),
            ("status", "show_status", "Display framework status"),
        ];

        for (cmd, action, desc) in default_mappings {
            let _ = conn.execute(
                "INSERT OR IGNORE INTO command_mappings (command, action, description) VALUES (?1, ?2, ?3)",
                params![cmd, action, desc],
            );
        }

        Ok(())
    }

    /// Add a command mapping
    pub fn add_mapping(&self, mapping: CommandMapping) -> FrameworkResult<()> {
        let conn = self.conn.lock();
        
        conn.execute(
            "INSERT OR REPLACE INTO command_mappings (command, action, description, enabled) VALUES (?1, ?2, ?3, ?4)",
            params![
                mapping.command,
                mapping.action,
                mapping.description,
                mapping.enabled as i32,
            ],
        ).map_err(|e| FrameworkError::SystemError(format!(
            "Failed to add mapping: {}", e
        )))?;

        Ok(())
    }

    /// Get command mapping
    pub fn get_mapping(&self, command: &str) -> FrameworkResult<Option<CommandMapping>> {
        let conn = self.conn.lock();
        
        let mut stmt = conn.prepare(
            "SELECT command, action, description, enabled FROM command_mappings WHERE command = ?1"
        ).map_err(|e| FrameworkError::SystemError(format!(
            "Failed to prepare statement: {}", e
        )))?;

        let result = stmt.query_row(params![command], |row| {
            Ok(CommandMapping {
                command: row.get(0)?,
                action: row.get(1)?,
                description: row.get(2)?,
                enabled: row.get::<_, i32>(3)? != 0,
            })
        });

        match result {
            Ok(mapping) => Ok(Some(mapping)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(FrameworkError::SystemError(format!(
                "Failed to get mapping: {}", e
            ))),
        }
    }

    /// Get all command mappings
    pub fn get_all_mappings(&self) -> FrameworkResult<Vec<CommandMapping>> {
        let conn = self.conn.lock();
        
        let mut stmt = conn.prepare(
            "SELECT command, action, description, enabled FROM command_mappings ORDER BY command"
        ).map_err(|e| FrameworkError::SystemError(format!(
            "Failed to prepare statement: {}", e
        )))?;

        let mappings = stmt.query_map([], |row| {
            Ok(CommandMapping {
                command: row.get(0)?,
                action: row.get(1)?,
                description: row.get(2)?,
                enabled: row.get::<_, i32>(3)? != 0,
            })
        }).map_err(|e| FrameworkError::SystemError(format!(
            "Failed to query mappings: {}", e
        )))?
        .filter_map(|r| r.ok())
        .collect();

        Ok(mappings)
    }

    /// Remove a command mapping
    pub fn remove_mapping(&self, command: &str) -> FrameworkResult<()> {
        let conn = self.conn.lock();
        
        conn.execute(
            "DELETE FROM command_mappings WHERE command = ?1",
            params![command],
        ).map_err(|e| FrameworkError::SystemError(format!(
            "Failed to remove mapping: {}", e
        )))?;

        Ok(())
    }

    /// Update command mapping
    pub fn update_mapping(&self, mapping: CommandMapping) -> FrameworkResult<()> {
        let conn = self.conn.lock();
        
        conn.execute(
            "UPDATE command_mappings SET action = ?1, description = ?2, enabled = ?3, updated_at = CURRENT_TIMESTAMP WHERE command = ?4",
            params![
                mapping.action,
                mapping.description,
                mapping.enabled as i32,
                mapping.command,
            ],
        ).map_err(|e| FrameworkError::SystemError(format!(
            "Failed to update mapping: {}", e
        )))?;

        Ok(())
    }

    /// Get database path
    pub fn path(&self) -> &Path {
        &self.db_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_database_creation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(&db_path).unwrap();
        assert!(db_path.exists());
    }

    #[test]
    fn test_add_and_get_mapping() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(&db_path).unwrap();

        let mapping = CommandMapping {
            command: "test".to_string(),
            action: "test_action".to_string(),
            description: "Test command".to_string(),
            enabled: true,
        };

        db.add_mapping(mapping).unwrap();
        let retrieved = db.get_mapping("test").unwrap().unwrap();
        assert_eq!(retrieved.command, "test");
        assert_eq!(retrieved.action, "test_action");
    }

    #[test]
    fn test_get_all_mappings() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let db = Database::new(&db_path).unwrap();

        let mappings = db.get_all_mappings().unwrap();
        assert!(!mappings.is_empty()); // Default mappings should exist
    }
}
