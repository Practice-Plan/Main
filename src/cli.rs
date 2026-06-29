//! CLI Command Parsing Module
//! 
//! Provides command-line interface parsing and handling functionality

use clap::{Parser, Subcommand};

/// PPC Command-line Tool
#[derive(Parser, Debug)]
#[command(name = "PPC")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Practice Plan Controller - Base Framework Command-line Tool")]
#[command(long_about = None)]
#[command(disable_version_flag = true)]      // Disable auto-generated --version
#[command(disable_help_flag = true)]         // Disable auto-generated --help flag
#[command(disable_help_subcommand = true)]   // Disable auto-generated help subcommand
pub struct Cli {
    /// Command
    #[command(subcommand)]
    pub command: Option<Commands>,
    
    /// Display version information (supports -v, -version, --version)
    #[arg(short = 'v', long = "version")]
    pub version_flag: bool,
    
    /// Display help information (supports -h, --help)
    #[arg(short = 'h', long = "help")]
    pub help_flag: bool,
}

/// Available Commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Display version information
    #[command(alias = "ver")]
    Version,
    
    /// Display help information
    #[command(alias = "h")]
    Help,
    
    /// Display system information (calls sysinfo DLL)
    Sysinfo,
    
    /// Display framework status
    Status,
    
    /// Manage command mappings
    #[command(subcommand)]
    Command(CommandCommands),
    
    /// Load DLL
    Load {
        /// DLL name
        name: String,
        /// DLL path
        path: String,
    },
    
    /// Unload DLL
    Unload {
        /// DLL name
        name: String,
    },
    
    /// List loaded DLLs
    List,
}

/// Command Management Subcommands
#[derive(Subcommand, Debug)]
pub enum CommandCommands {
    /// Add command mapping
    Add {
        /// Command name
        command: String,
        /// Action name
        action: String,
        /// Description
        #[arg(short, long, default_value = "")]
        description: String,
    },
    
    /// Remove command mapping
    Remove {
        /// Command name
        command: String,
    },
    
    /// List all command mappings
    List,
    
    /// Get command details
    Get {
        /// Command name
        command: String,
    },
}

impl Cli {
    /// Parse command-line arguments
    pub fn parse_args() -> Self {
        Cli::parse()
    }
    
    /// Get the name of the command to execute
    pub fn get_command_name(&self) -> Option<String> {
        if self.version_flag {
            return Some("version".to_string());
        }
        
        if self.help_flag {
            return Some("help".to_string());
        }
        
        match &self.command {
            Some(Commands::Version) => Some("version".to_string()),
            Some(Commands::Help) => Some("help".to_string()),
            Some(Commands::Sysinfo) => Some("sysinfo".to_string()),
            Some(Commands::Status) => Some("status".to_string()),
            Some(Commands::Command(_)) => Some("command".to_string()),
            Some(Commands::Load { .. }) => Some("load".to_string()),
            Some(Commands::Unload { .. }) => Some("unload".to_string()),
            Some(Commands::List) => Some("list".to_string()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_version_flag() {
        let cli = Cli {
            command: None,
            version_flag: true,
            help_flag: false,
        };
        assert_eq!(cli.get_command_name(), Some("version".to_string()));
    }
    
    #[test]
    fn test_cli_help_flag() {
        let cli = Cli {
            command: None,
            version_flag: false,
            help_flag: true,
        };
        assert_eq!(cli.get_command_name(), Some("help".to_string()));
    }
    
    #[test]
    fn test_cli_version_command() {
        let cli = Cli {
            command: Some(Commands::Version),
            version_flag: false,
            help_flag: false,
        };
        assert_eq!(cli.get_command_name(), Some("version".to_string()));
    }
}
