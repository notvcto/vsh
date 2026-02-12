use crate::error::{Result, VshError};
use crate::parser::{Command, Intent};
use colored::*;
use std::fs;
use std::path::Path;

/// Execute a built-in command
pub fn execute_builtin(cmd: Command) -> Result<()> {
    match cmd.intent {
        Intent::Copy => execute_copy(&cmd),
        Intent::Move => execute_move(&cmd),
        Intent::Remove => execute_remove(&cmd),
        Intent::List => execute_list(&cmd),
        Intent::ChangeDirectory => execute_cd(&cmd),
    }
}

/// Execute copy command
fn execute_copy(cmd: &Command) -> Result<()> {
    let source = cmd
        .args
        .source
        .as_ref()
        .ok_or_else(|| VshError::InvalidSyntax("Missing source".to_string()))?;

    let dest = cmd
        .args
        .destination
        .as_ref()
        .ok_or_else(|| VshError::InvalidSyntax("Missing destination".to_string()))?;

    let source_path = Path::new(source);
    let dest_path = Path::new(dest);

    // Check if source exists
    if !source_path.exists() {
        return Err(VshError::FileNotFound(source.clone()));
    }

    // Check if source is a file or directory
    if source_path.is_file() {
        // Determine final destination path
        let final_dest =
            if dest_path.is_dir() {
                // Copy into directory with same filename
                dest_path.join(source_path.file_name().ok_or_else(|| {
                    VshError::InvalidSyntax("Invalid source filename".to_string())
                })?)
            } else {
                dest_path.to_path_buf()
            };

        // Perform the copy
        fs::copy(source_path, &final_dest).map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                VshError::PermissionDenied(source.clone())
            } else {
                VshError::IoError(e)
            }
        })?;

        println!(
            "{} {} {} {}",
            "✓".green().bold(),
            "Copied".green(),
            source,
            format!("→ {}", final_dest.display()).bright_black()
        );
    } else if source_path.is_dir() {
        // For now, don't support recursive copy in MVP
        return Err(VshError::ExecutionError(
            "Directory copy not yet implemented. Use -r flag (coming soon)".to_string(),
        ));
    }

    Ok(())
}

/// Execute move command
fn execute_move(cmd: &Command) -> Result<()> {
    let source = cmd
        .args
        .source
        .as_ref()
        .ok_or_else(|| VshError::InvalidSyntax("Missing source".to_string()))?;

    let dest = cmd
        .args
        .destination
        .as_ref()
        .ok_or_else(|| VshError::InvalidSyntax("Missing destination".to_string()))?;

    let source_path = Path::new(source);
    let dest_path = Path::new(dest);

    if !source_path.exists() {
        return Err(VshError::FileNotFound(source.clone()));
    }

    // Determine final destination
    let final_dest = if dest_path.is_dir() {
        dest_path.join(
            source_path
                .file_name()
                .ok_or_else(|| VshError::InvalidSyntax("Invalid source filename".to_string()))?,
        )
    } else {
        dest_path.to_path_buf()
    };

    // Perform the move
    fs::rename(source_path, &final_dest)?;

    println!(
        "{} {} {} {}",
        "✓".green().bold(),
        "Moved".green(),
        source,
        format!("→ {}", final_dest.display()).bright_black()
    );

    Ok(())
}

/// Execute remove command
fn execute_remove(cmd: &Command) -> Result<()> {
    let source = cmd
        .args
        .source
        .as_ref()
        .ok_or_else(|| VshError::InvalidSyntax("Missing file to remove".to_string()))?;

    let path = Path::new(source);

    if !path.exists() {
        return Err(VshError::FileNotFound(source.clone()));
    }

    // Safety check: ask for confirmation unless -f flag
    let force = cmd.args.flags.contains(&"-f".to_string());

    if !force {
        print!("{} Remove '{}'? [y/N]: ", "⚠".yellow(), source);
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        if !response.trim().eq_ignore_ascii_case("y") {
            println!("{}", "Cancelled".bright_black());
            return Ok(());
        }
    }

    // Perform the removal
    if path.is_file() {
        fs::remove_file(path)?;
    } else if path.is_dir() {
        fs::remove_dir_all(path)?;
    }

    println!("{} {} {}", "✓".green().bold(), "Removed".green(), source);

    Ok(())
}

/// Execute list command
fn execute_list(cmd: &Command) -> Result<()> {
    let path = cmd.args.path.as_deref().unwrap_or(".");

    let dir_path = Path::new(path);

    if !dir_path.exists() {
        return Err(VshError::FileNotFound(path.to_string()));
    }

    if !dir_path.is_dir() {
        return Err(VshError::ExecutionError(format!(
            "'{}' is not a directory",
            path
        )));
    }

    let entries = fs::read_dir(dir_path)?;

    println!("{}", format!("Contents of {}:", path).bright_cyan());
    println!();

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            println!("  📁 {}/", file_name_str.bright_blue());
        } else {
            println!("  📄 {}", file_name_str);
        }
    }

    println!();

    Ok(())
}

/// Execute change directory command
fn execute_cd(cmd: &Command) -> Result<()> {
    let path = cmd.args.path.as_deref().unwrap_or("~");

    // Expand ~ to home directory
    let expanded_path = if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            path.replacen('~', &home.to_string_lossy(), 1)
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    };

    let target_path = Path::new(&expanded_path);

    if !target_path.exists() {
        return Err(VshError::FileNotFound(expanded_path.clone()));
    }

    if !target_path.is_dir() {
        return Err(VshError::ExecutionError(format!(
            "'{}' is not a directory",
            expanded_path
        )));
    }

    // Change directory
    std::env::set_current_dir(target_path)?;

    println!(
        "{} Changed directory to {}",
        "✓".green().bold(),
        target_path.display()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{CommandArgs, SyntaxStyle};
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_copy_file() {
        let dir = tempdir().unwrap();
        let source_path = dir.path().join("source.txt");
        let dest_path = dir.path().join("dest.txt");

        // Create source file
        File::create(&source_path).unwrap();

        let cmd = Command {
            intent: Intent::Copy,
            args: CommandArgs {
                source: Some(source_path.to_string_lossy().to_string()),
                destination: Some(dest_path.to_string_lossy().to_string()),
                path: None,
                flags: vec![],
                extra: vec![],
            },
            syntax_used: SyntaxStyle::Terse,
        };

        execute_copy(&cmd).unwrap();

        assert!(dest_path.exists());
    }

    #[test]
    fn test_copy_nonexistent_file() {
        let cmd = Command {
            intent: Intent::Copy,
            args: CommandArgs {
                source: Some("nonexistent.txt".to_string()),
                destination: Some("dest.txt".to_string()),
                path: None,
                flags: vec![],
                extra: vec![],
            },
            syntax_used: SyntaxStyle::Terse,
        };

        let result = execute_copy(&cmd);
        assert!(result.is_err());
    }
}
