mod syntax;
mod tokenizer;
mod translator;

use crate::error::{Result, VshError};

/// Represents the intent/action the user wants to perform
#[derive(Debug, Clone, PartialEq)]
pub enum Intent {
    Copy,
    Move,
    Remove,
    List,
    ChangeDirectory,
    // Add more as we implement them
}

/// Represents the detected syntax style
#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxStyle {
    Terse,   // cp, mv, rm
    Verbose, // copy, move, remove
    Named,   // copy source=x dest=y
}

/// A parsed command ready for execution
#[derive(Debug, Clone)]
pub struct Command {
    pub intent: Intent,
    pub args: CommandArgs,
    pub syntax_used: SyntaxStyle,
}

/// Command arguments in canonical form
#[derive(Debug, Clone)]
pub struct CommandArgs {
    pub source: Option<String>,
    pub destination: Option<String>,
    pub path: Option<String>,
    pub flags: Vec<String>,
    pub extra: Vec<String>,
}

impl CommandArgs {
    pub fn new() -> Self {
        Self {
            source: None,
            destination: None,
            path: None,
            flags: Vec::new(),
            extra: Vec::new(),
        }
    }
}

impl Default for CommandArgs {
    fn default() -> Self {
        Self::new()
    }
}

/// Main entry point for parsing commands
pub fn parse_command(input: &str) -> Result<Command> {
    // Step 1: Tokenize
    let tokens = tokenizer::tokenize(input)?;

    if tokens.is_empty() {
        return Err(VshError::InvalidSyntax("Empty command".to_string()));
    }

    // Step 2: Detect syntax style
    let syntax = syntax::detect_syntax(&tokens);

    // Step 3: Identify intent
    let intent = syntax::identify_intent(&tokens[0].value)?;

    // Step 4: Extract arguments based on syntax and intent
    let args = translator::extract_args(&tokens, &syntax, &intent)?;

    Ok(Command {
        intent,
        args,
        syntax_used: syntax,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_terse_copy() {
        let cmd = parse_command("cp file.txt backup/").unwrap();
        assert_eq!(cmd.intent, Intent::Copy);
        assert_eq!(cmd.syntax_used, SyntaxStyle::Terse);
        assert_eq!(cmd.args.source, Some("file.txt".to_string()));
        assert_eq!(cmd.args.destination, Some("backup/".to_string()));
    }

    #[test]
    fn test_parse_verbose_copy() {
        let cmd = parse_command("copy file.txt to backup/").unwrap();
        assert_eq!(cmd.intent, Intent::Copy);
        assert_eq!(cmd.syntax_used, SyntaxStyle::Verbose);
        assert_eq!(cmd.args.source, Some("file.txt".to_string()));
        assert_eq!(cmd.args.destination, Some("backup/".to_string()));
    }

    #[test]
    fn test_parse_named_copy() {
        let cmd = parse_command("copy source=file.txt destination=backup/").unwrap();
        assert_eq!(cmd.intent, Intent::Copy);
        assert_eq!(cmd.syntax_used, SyntaxStyle::Named);
        assert_eq!(cmd.args.source, Some("file.txt".to_string()));
        assert_eq!(cmd.args.destination, Some("backup/".to_string()));
    }
}
