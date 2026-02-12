use super::{CommandArgs, Intent, SyntaxStyle};
use crate::error::{Result, VshError};
use crate::parser::tokenizer::Token;

/// Extract arguments from tokens based on syntax style and intent
pub fn extract_args(
    tokens: &[Token],
    syntax: &SyntaxStyle,
    intent: &Intent,
) -> Result<CommandArgs> {
    match syntax {
        SyntaxStyle::Named => extract_named_args(tokens, intent),
        SyntaxStyle::Verbose => extract_verbose_args(tokens, intent),
        SyntaxStyle::Terse => extract_terse_args(tokens, intent),
    }
}

/// Extract arguments from named parameter syntax (key=value)
fn extract_named_args(tokens: &[Token], intent: &Intent) -> Result<CommandArgs> {
    let mut args = CommandArgs::new();

    for token in tokens.iter().skip(1) {
        // Skip command itself
        if let Some((key, value)) = token.value.split_once('=') {
            match key {
                "source" | "src" | "from" => args.source = Some(value.to_string()),
                "destination" | "dest" | "to" | "target" => {
                    args.destination = Some(value.to_string())
                }
                "path" | "directory" | "dir" => args.path = Some(value.to_string()),
                _ => args.extra.push(token.value.clone()),
            }
        } else {
            // Not a key=value pair, treat as extra arg
            args.extra.push(token.value.clone());
        }
    }

    validate_args(&args, intent)?;
    Ok(args)
}

/// Extract arguments from verbose syntax (with connector words)
fn extract_verbose_args(tokens: &[Token], intent: &Intent) -> Result<CommandArgs> {
    let mut args = CommandArgs::new();
    let mut i = 1; // Skip command

    while i < tokens.len() {
        let token = &tokens[i];

        match token.value.as_str() {
            "to" | "into" => {
                // Next token is destination
                if i + 1 < tokens.len() {
                    args.destination = Some(tokens[i + 1].value.clone());
                    i += 2;
                } else {
                    return Err(VshError::InvalidSyntax(
                        "Expected destination after 'to'".to_string(),
                    ));
                }
            }
            "from" => {
                // Next token is source
                if i + 1 < tokens.len() {
                    args.source = Some(tokens[i + 1].value.clone());
                    i += 2;
                } else {
                    return Err(VshError::InvalidSyntax(
                        "Expected source after 'from'".to_string(),
                    ));
                }
            }
            "in" | "at" => {
                // Next token is path
                if i + 1 < tokens.len() {
                    args.path = Some(tokens[i + 1].value.clone());
                    i += 2;
                } else {
                    return Err(VshError::InvalidSyntax(
                        "Expected path after 'in'".to_string(),
                    ));
                }
            }
            _ => {
                // If we haven't found source yet, this is it
                if args.source.is_none() && requires_source(intent) {
                    args.source = Some(token.value.clone());
                } else if args.path.is_none() && requires_path(intent) {
                    args.path = Some(token.value.clone());
                } else {
                    args.extra.push(token.value.clone());
                }
                i += 1;
            }
        }
    }

    validate_args(&args, intent)?;
    Ok(args)
}

/// Extract arguments from terse syntax (positional)
fn extract_terse_args(tokens: &[Token], intent: &Intent) -> Result<CommandArgs> {
    let mut args = CommandArgs::new();

    match intent {
        Intent::Copy | Intent::Move => {
            // Format: cp source dest
            if tokens.len() < 3 {
                return Err(VshError::InvalidSyntax(
                    "Expected: <command> <source> <destination>".to_string(),
                ));
            }
            args.source = Some(tokens[1].value.clone());
            args.destination = Some(tokens[2].value.clone());

            // Rest are flags or extra args
            for token in tokens.iter().skip(3) {
                if token.value.starts_with('-') {
                    args.flags.push(token.value.clone());
                } else {
                    args.extra.push(token.value.clone());
                }
            }
        }
        Intent::Remove => {
            // Format: rm file [file2 file3...]
            if tokens.len() < 2 {
                return Err(VshError::InvalidSyntax(
                    "Expected: <command> <file>".to_string(),
                ));
            }
            args.source = Some(tokens[1].value.clone());

            // Additional files or flags
            for token in tokens.iter().skip(2) {
                if token.value.starts_with('-') {
                    args.flags.push(token.value.clone());
                } else {
                    args.extra.push(token.value.clone());
                }
            }
        }
        Intent::List | Intent::ChangeDirectory => {
            // Format: ls [path]
            if tokens.len() > 1 {
                args.path = Some(tokens[1].value.clone());
            } else {
                args.path = Some(".".to_string()); // Default to current dir
            }

            // Rest are flags
            for token in tokens.iter().skip(2) {
                if token.value.starts_with('-') {
                    args.flags.push(token.value.clone());
                } else {
                    args.extra.push(token.value.clone());
                }
            }
        }
    }

    validate_args(&args, intent)?;
    Ok(args)
}

/// Check if intent requires a source argument
fn requires_source(intent: &Intent) -> bool {
    matches!(intent, Intent::Copy | Intent::Move | Intent::Remove)
}

/// Check if intent requires a path argument
fn requires_path(intent: &Intent) -> bool {
    matches!(intent, Intent::List | Intent::ChangeDirectory)
}

/// Validate that required arguments are present
fn validate_args(args: &CommandArgs, intent: &Intent) -> Result<()> {
    match intent {
        Intent::Copy | Intent::Move => {
            if args.source.is_none() {
                return Err(VshError::InvalidSyntax("Missing source".to_string()));
            }
            if args.destination.is_none() {
                return Err(VshError::InvalidSyntax("Missing destination".to_string()));
            }
        }
        Intent::Remove => {
            if args.source.is_none() {
                return Err(VshError::InvalidSyntax("Missing file to remove".to_string()));
            }
        }
        Intent::List | Intent::ChangeDirectory => {
            // Path is optional, defaults handled elsewhere
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_token(s: &str) -> Token {
        Token {
            value: s.to_string(),
            is_quoted: false,
        }
    }

    #[test]
    fn test_extract_terse_copy() {
        let tokens = vec![
            make_token("cp"),
            make_token("file.txt"),
            make_token("backup/"),
        ];
        let args = extract_terse_args(&tokens, &Intent::Copy).unwrap();
        assert_eq!(args.source, Some("file.txt".to_string()));
        assert_eq!(args.destination, Some("backup/".to_string()));
    }

    #[test]
    fn test_extract_verbose_copy() {
        let tokens = vec![
            make_token("copy"),
            make_token("file.txt"),
            make_token("to"),
            make_token("backup/"),
        ];
        let args = extract_verbose_args(&tokens, &Intent::Copy).unwrap();
        assert_eq!(args.source, Some("file.txt".to_string()));
        assert_eq!(args.destination, Some("backup/".to_string()));
    }

    #[test]
    fn test_extract_named_copy() {
        let tokens = vec![
            make_token("copy"),
            make_token("source=file.txt"),
            make_token("destination=backup/"),
        ];
        let args = extract_named_args(&tokens, &Intent::Copy).unwrap();
        assert_eq!(args.source, Some("file.txt".to_string()));
        assert_eq!(args.destination, Some("backup/".to_string()));
    }
}
