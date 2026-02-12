use anyhow::Result;
use clap::Parser;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use vsh::executor::execute_command;
use vsh::parser::parse_command;

#[derive(Parser)]
#[command(name = "vsh")]
#[command(about = "A shell for everyone, built by everyone", long_about = None)]
struct Cli {
    /// Script file to execute
    #[arg(short, long)]
    script: Option<String>,

    /// Command to execute
    #[arg(short, long)]
    command: Option<String>,

    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Execute single command if provided
    if let Some(cmd) = cli.command {
        return execute_single_command(&cmd, cli.debug);
    }

    // Execute script if provided
    if let Some(script_path) = cli.script {
        return execute_script(&script_path, cli.debug);
    }

    // Otherwise, start REPL
    start_repl(cli.debug)
}

fn execute_single_command(command: &str, debug: bool) -> Result<()> {
    if debug {
        println!("{} {}", "DEBUG:".yellow(), command);
    }

    match parse_command(command) {
        Ok(cmd) => {
            if debug {
                println!("{} {:?}", "Parsed:".cyan(), cmd);
            }
            execute_command(cmd)?;
        }
        Err(e) => {
            eprintln!("{} {}", "✗ Parse error:".red(), e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn execute_script(_script_path: &str, _debug: bool) -> Result<()> {
    // TODO: Implement script execution in Phase 3
    eprintln!("Script execution not yet implemented");
    std::process::exit(1);
}

fn start_repl(debug: bool) -> Result<()> {
    println!("{}", "VSH - Vic's Shell v0.1.0".bright_cyan().bold());
    println!("{}", "A shell for everyone, built by everyone.".bright_black());
    println!("{}\n", "Type 'help' for commands, 'exit' to quit.".bright_black());

    let mut rl = DefaultEditor::new()?;

    // Load history if it exists
    let history_path = dirs::home_dir()
        .map(|mut p| {
            p.push(".vsh_history");
            p
        })
        .expect("Could not find home directory");

    let _ = rl.load_history(&history_path);

    loop {
        // Get username and hostname
        let username = whoami::username();
        let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "localhost".to_string());
        
        // Get current directory for prompt
        let current_dir = std::env::current_dir()
            .ok()
            .and_then(|path| {
                // Try to replace home directory with ~
                if let Some(home) = dirs::home_dir() {
                    if path == home {
                        return Some("~".to_string());
                    } else if let Ok(stripped) = path.strip_prefix(&home) {
                        return Some(format!("~/{}", stripped.display()));
                    }
                }
                Some(path.display().to_string())
            })
            .unwrap_or_else(|| "?".to_string());
        
        let prompt = format!("{} ", format!("{}@{}:{}$", username, hostname, current_dir).bright_green().bold());
        
        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim();
                
                if line.is_empty() {
                    continue;
                }

                // Add to history
                let _ = rl.add_history_entry(line);

                // Handle built-in commands
                match line {
                    "exit" | "quit" => {
                        println!("{}", "Goodbye! 👋".bright_cyan());
                        break;
                    }
                    "help" => {
                        print_help();
                        continue;
                    }
                    "pwd" => {
                        match std::env::current_dir() {
                            Ok(path) => println!("{}", path.display()),
                            Err(e) => eprintln!("{} {}", "✗ Error:".red(), e),
                        }
                        continue;
                    }
                    _ => {}
                }

                // Parse and execute command
                if debug {
                    println!("{} {}", "DEBUG:".yellow(), line);
                }

                match parse_command(line) {
                    Ok(cmd) => {
                        if debug {
                            println!("{} {:?}", "Parsed:".cyan(), cmd);
                        }

                        if let Err(e) = execute_command(cmd) {
                            eprintln!("{} {}", "✗ Error:".red(), e);
                        }
                    }
                    Err(e) => {
                        eprintln!("{} {}", "✗ Parse error:".red(), e);
                        print_suggestion(line);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "Goodbye! 👋".bright_cyan());
                break;
            }
            Err(err) => {
                eprintln!("{} {:?}", "Error:".red(), err);
                break;
            }
        }
    }

    // Save history
    let _ = rl.save_history(&history_path);

    Ok(())
}

fn print_help() {
    println!("{}", "VSH Commands:".bright_cyan().bold());
    println!();
    println!("  {}  Copy files", "copy <source> to <dest>".bright_yellow());
    println!("  {}", "       (or: cp <source> <dest>)".bright_black());
    println!("  {}", "       (or: copy source=<file> destination=<dest>)".bright_black());
    println!();
    println!("  {}  Move/rename files", "move <source> to <dest>".bright_yellow());
    println!("  {}", "       (or: mv <source> <dest>)".bright_black());
    println!();
    println!("  {}  Remove files", "remove <file>".bright_yellow());
    println!("  {}", "        (or: rm <file>)".bright_black());
    println!("  {}", "        (or: delete <file>)".bright_black());
    println!();
    println!("  {}  List files", "list [path]".bright_yellow());
    println!("  {}", "              (or: ls [path])".bright_black());
    println!();
    println!("  {}  Change directory", "cd <path>".bright_yellow());
    println!("  {}", "                  (or: change-directory <path>)".bright_black());
    println!("  {}", "                  (or: goto <path>)".bright_black());
    println!();
    println!("  {}  Print working directory", "pwd".bright_yellow());
    println!();
    println!("  {}                Exit VSH", "exit".bright_yellow());
    println!("  {}                Show this help", "help".bright_yellow());
    println!();
    println!("{}", "Tip: All commands support terse, verbose, and named syntax!".bright_black());
}

fn print_suggestion(input: &str) {
    // Simple suggestion system - can be enhanced later
    if input.starts_with("cp ") {
        println!("{} Try: copy <source> to <dest>", "Suggestion:".cyan());
    } else if input.starts_with("ls") {
        println!("{} Try: list [path]", "Suggestion:".cyan());
    } else if input.starts_with("mv ") {
        println!("{} Try: move <source> to <dest>", "Suggestion:".cyan());
    } else if input.starts_with("rm ") {
        println!("{} Try: remove <file>", "Suggestion:".cyan());
    }
}
