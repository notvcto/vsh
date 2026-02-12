use crate::error::{Result, VshError};

/// A token in the command
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub value: String,
    pub is_quoted: bool,
}

/// Tokenize input string into individual tokens
/// Handles quotes, escapes, and basic splitting
pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';
    let mut escape_next = false;

    for ch in input.chars() {
        if escape_next {
            current_token.push(ch);
            escape_next = false;
            continue;
        }

        match ch {
            '\\' => {
                escape_next = true;
            }
            '"' | '\'' => {
                if in_quotes && ch == quote_char {
                    // End quote
                    in_quotes = false;
                    if !current_token.is_empty() {
                        tokens.push(Token {
                            value: current_token.clone(),
                            is_quoted: true,
                        });
                        current_token.clear();
                    }
                } else if !in_quotes {
                    // Start quote
                    in_quotes = true;
                    quote_char = ch;
                    if !current_token.is_empty() {
                        tokens.push(Token {
                            value: current_token.clone(),
                            is_quoted: false,
                        });
                        current_token.clear();
                    }
                } else {
                    // Quote inside different quote type
                    current_token.push(ch);
                }
            }
            ' ' | '\t' => {
                if in_quotes {
                    current_token.push(ch);
                } else if !current_token.is_empty() {
                    tokens.push(Token {
                        value: current_token.clone(),
                        is_quoted: false,
                    });
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(ch);
            }
        }
    }

    // Add final token
    if !current_token.is_empty() {
        tokens.push(Token {
            value: current_token,
            is_quoted: in_quotes,
        });
    }

    if in_quotes {
        return Err(VshError::ParseError("Unclosed quote".to_string()));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenize() {
        let tokens = tokenize("copy file.txt backup/").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].value, "copy");
        assert_eq!(tokens[1].value, "file.txt");
        assert_eq!(tokens[2].value, "backup/");
    }

    #[test]
    fn test_quoted_tokenize() {
        let tokens = tokenize("copy \"my file.txt\" backup/").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[1].value, "my file.txt");
        assert!(tokens[1].is_quoted);
    }

    #[test]
    fn test_named_params() {
        let tokens = tokenize("copy source=file.txt destination=backup/").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[1].value, "source=file.txt");
    }

    #[test]
    fn test_unclosed_quote() {
        let result = tokenize("copy \"file.txt");
        assert!(result.is_err());
    }
}
