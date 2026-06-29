//! PPC Main Entry
//! 
//! Practice Plan Controller - Command-line tool main program

use base_framework::cli::{Cli, Commands, CommandCommands};
use base_framework::database::{Database, CommandMapping};
use base_framework::dll_loader::DllLoader;
use base_framework::error::FrameworkResult;
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;

/// Application context
struct AppContext {
    db: Database,
    dll_loader: DllLoader,
}

impl AppContext {
    fn new() -> FrameworkResult<Self> {
        // Get data directory
        let data_dir = get_data_dir()?;
        std::fs::create_dir_all(&data_dir).map_err(|e| {
            base_framework::error::FrameworkError::InitError(format!(
                "Failed to create data directory: {}", e
            ))
        })?;

        let db_path = data_dir.join("ppc_commands.db");
        let db = Database::new(&db_path)?;
        let dll_loader = DllLoader::new();

        // Add default DLL search path
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        
        dll_loader.add_search_path(&exe_dir);
        dll_loader.add_search_path(exe_dir.join("..").join("dll").join("target").join("release"));

        Ok(Self { db, dll_loader })
    }
}

/// Get data directory
fn get_data_dir() -> FrameworkResult<PathBuf> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    
    Ok(exe_dir.join(".ppc_data"))
}

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> FrameworkResult<()> {
    let ctx = AppContext::new()?;

    // Handle -version flag
    if cli.version_flag {
        return cmd_version();
    }

    match cli.command {
        Some(Commands::Version) => cmd_version(),
        Some(Commands::Help) => cmd_help(),
        Some(Commands::Sysinfo) => cmd_sysinfo(&ctx),
        Some(Commands::Status) => cmd_status(&ctx),
        Some(Commands::Command(sub)) => cmd_command(&ctx, sub),
        Some(Commands::Load { name, path }) => cmd_load(&ctx, &name, &path),
        Some(Commands::Unload { name }) => cmd_unload(&ctx, &name),
        Some(Commands::List) => cmd_list(&ctx),
        None => cmd_help(),
    }
}

/// Display version information
fn cmd_version() -> FrameworkResult<()> {
    println!("PPC (Practice Plan Controller) v{}", env!("CARGO_PKG_VERSION"));
    println!("Base Framework v{}", base_framework::VERSION);
    Ok(())
}

/// Display help information
fn cmd_help() -> FrameworkResult<()> {
    println!("PPC - Practice Plan Controller v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE:");
    println!("    PPC [COMMAND] [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    version, -version, -v    Display version information");
    println!("    help                     Display help information");
    println!("    sysinfo                  Display system information");
    println!("    status                   Display framework status");
    println!("    command add <cmd> <act>  Add command mapping");
    println!("    command remove <cmd>     Remove command mapping");
    println!("    command list             List all command mappings");
    println!("    command get <cmd>        Get command details");
    println!("    load <name> <path>       Load DLL");
    println!("    unload <name>            Unload DLL");
    println!("    list                     List loaded DLLs");
    Ok(())
}

/// Display system information
fn cmd_sysinfo(ctx: &AppContext) -> FrameworkResult<()> {
    let dll_name = "sysinfo";
    let dll_file = "sysinfo.dll";
    
    if !ctx.dll_loader.is_loaded(dll_name) {
        let search_paths = ctx.dll_loader.get_search_paths();
        let mut found = false;
        
        for search_path in &search_paths {
            let full_path = search_path.join(dll_file);
            if full_path.exists() {
                ctx.dll_loader.load(dll_name, &full_path)?;
                found = true;
                break;
            }
        }
        
        if !found {
            eprintln!("Failed to load {}: DLL not found in search paths", dll_file);
            eprintln!("Please ensure {} is in the search path.", dll_file);
            return Ok(());
        }
    }

    match ctx.dll_loader.call_string_fn(dll_name, "sysinfo_get_summary") {
        Ok(summary) => {
            print!("{}", summary);
        }
        Err(e) => {
            eprintln!("Failed to get system info: {}", e);
        }
    }

    Ok(())
}

/// Display framework status
fn cmd_status(ctx: &AppContext) -> FrameworkResult<()> {
    println!("=== PPC Framework Status ===");
    println!();
    
    // Database status
    println!("Database:");
    println!("  Path: {}", ctx.db.path().display());
    
    let mappings = ctx.db.get_all_mappings()?;
    println!("  Commands: {}", mappings.len());
    println!();
    
    // DLL status
    println!("DLL Loader:");
    let loaded = ctx.dll_loader.list_loaded();
    println!("  Loaded DLLs: {}", loaded.len());
    for dll in &loaded {
        println!("    - {}", dll);
    }
    
    Ok(())
}

/// Command management
fn cmd_command(ctx: &AppContext, sub: CommandCommands) -> FrameworkResult<()> {
    match sub {
        CommandCommands::Add { command, action, description } => {
            let mapping = CommandMapping {
                command: command.clone(),
                action,
                description,
                enabled: true,
            };
            ctx.db.add_mapping(mapping)?;
            println!("Command '{}' added successfully.", command);
        }
        CommandCommands::Remove { command } => {
            ctx.db.remove_mapping(&command)?;
            println!("Command '{}' removed successfully.", command);
        }
        CommandCommands::List => {
            let mappings = ctx.db.get_all_mappings()?;
            println!("=== Command Mappings ===");
            println!("{:<20} {:<25} {:<30} {}", "Command", "Action", "Description", "Enabled");
            println!("{}", "-".repeat(80));
            for m in mappings {
                println!(
                    "{:<20} {:<25} {:<30} {}",
                    m.command,
                    m.action,
                    m.description,
                    if m.enabled { "Yes" } else { "No" }
                );
            }
        }
        CommandCommands::Get { command } => {
            match ctx.db.get_mapping(&command)? {
                Some(m) => {
                    println!("Command:     {}", m.command);
                    println!("Action:      {}", m.action);
                    println!("Description: {}", m.description);
                    println!("Enabled:     {}", if m.enabled { "Yes" } else { "No" });
                }
                None => {
                    println!("Command '{}' not found.", command);
                }
            }
        }
    }
    Ok(())
}

/// Load DLL
fn cmd_load(ctx: &AppContext, name: &str, path: &str) -> FrameworkResult<()> {
    ctx.dll_loader.load(name, path)?;
    println!("DLL '{}' loaded from '{}'.", name, path);
    Ok(())
}

/// Unload DLL
fn cmd_unload(ctx: &AppContext, name: &str) -> FrameworkResult<()> {
    ctx.dll_loader.unload(name)?;
    println!("DLL '{}' unloaded.", name);
    Ok(())
}

/// List loaded DLLs
fn cmd_list(ctx: &AppContext) -> FrameworkResult<()> {
    let loaded = ctx.dll_loader.list_loaded();
    if loaded.is_empty() {
        println!("No DLLs loaded.");
    } else {
        println!("Loaded DLLs:");
        for dll in loaded {
            println!("  - {}", dll);
        }
    }
    Ok(())
}
