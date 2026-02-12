use super::{Intent, SyntaxStyle};
use crate::error::{Result, VshError};
use crate::parser::tokenizer::Token;

/// Detect which syntax style is being used
pub fn detect_syntax(tokens: &[Token]) -> SyntaxStyle {
    // Check for named parameters (key=value)
    for token in tokens {
        if token.value.contains('=') && !token.is_quoted {
            return SyntaxStyle::Named;
        }
    }

    // Check for verbose connector words (to, from, in, with, etc.)
    for token in tokens {
        if is_connector_word(&token.value) {
            return SyntaxStyle::Verbose;
        }
    }

    // Default to terse
    SyntaxStyle::Terse
}

/// Check if a word is a connector in verbose syntax
fn is_connector_word(word: &str) -> bool {
    matches!(
        word,
        "to" | "from" | "in" | "into" | "with" | "without" | "at" | "on"
    )
}

/// Identify the intent from the command word
pub fn identify_intent(command: &str) -> Result<Intent> {
    match command.to_lowercase().as_str() {
        // Copy
        "cp" | "copy" => Ok(Intent::Copy),

        // Move
        "mv" | "move" => Ok(Intent::Move),

        // Remove
        "rm" | "remove" | "delete" => Ok(Intent::Remove),

        // List
        "ls" | "list" | "dir" => Ok(Intent::List),

        // Change directory
        "cd" | "change-directory" | "goto" => Ok(Intent::ChangeDirectory),

        _ => Err(VshError::UnknownCommand(command.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_named_syntax() {
        let tokens = vec![
            Token {
                value: "copy".to_string(),
                is_quoted: false,
            },
            Token {
                value: "source=file.txt".to_string(),
                is_quoted: false,
            },
        ];
        assert_eq!(detect_syntax(&tokens), SyntaxStyle::Named);
    }

    #[test]
    fn test_detect_verbose_syntax() {
        let tokens = vec![
            Token {
                value: "copy".to_string(),
                is_quoted: false,
            },
            Token {
                value: "file.txt".to_string(),
                is_quoted: false,
            },
            Token {
                value: "to".to_string(),
                is_quoted: false,
            },
            Token {
                value: "backup/".to_string(),
                is_quoted: false,
            },
        ];
        assert_eq!(detect_syntax(&tokens), SyntaxStyle::Verbose);
    }

    #[test]
    fn test_detect_terse_syntax() {
        let tokens = vec![
            Token {
                value: "cp".to_string(),
                is_quoted: false,
            },
            Token {
                value: "file.txt".to_string(),
                is_quoted: false,
            },
            Token {
                value: "backup/".to_string(),
                is_quoted: false,
            },
        ];
        assert_eq!(detect_syntax(&tokens), SyntaxStyle::Terse);
    }

    #[test]
    fn test_identify_copy_commands() {
        assert_eq!(identify_intent("cp").unwrap(), Intent::Copy);
        assert_eq!(identify_intent("copy").unwrap(), Intent::Copy);
        assert_eq!(identify_intent("COPY").unwrap(), Intent::Copy);
    }

    #[test]
    fn test_identify_unknown_command() {
        assert!(identify_intent("unknown").is_err());
    }
}
