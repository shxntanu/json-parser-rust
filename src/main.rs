mod token;
use token::{Token, TokenType};

// lexer() function for generating a vector of tokens.
pub fn lexer(json: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_pointer = 0;

    while current_pointer < json.len() {
        if json
            .chars()
            .nth(current_pointer)
            .expect("Index out of bound")
            .is_numeric()
        {
            let mut value = String::new();
            let mut i = current_pointer;
            while json
                .chars()
                .nth(i)
                .expect("Index out of bound")
                .is_numeric()
            {
                value.push(json.chars().nth(i).expect("Index out of bound"));
                i += 1;
            }
            tokens.push(Token {
                token_type: TokenType::Number,
                value,
            });
            current_pointer = i;
        } else {
            match json.chars().nth(current_pointer) {
                Some('{') => {
                    tokens.push(Token {
                        token_type: TokenType::OpenBrace,
                        value: String::from('{'),
                    });
                    current_pointer += 1;
                }
                Some('}') => {
                    tokens.push(Token {
                        token_type: TokenType::CloseBrace,
                        value: String::from('}'),
                    });
                    current_pointer += 1;
                }
                Some('[') => {
                    tokens.push(Token {
                        token_type: TokenType::OpenSqBracket,
                        value: String::from('['),
                    });
                    current_pointer += 1;
                }
                Some(']') => {
                    tokens.push(Token {
                        token_type: TokenType::CloseSqBracket,
                        value: String::from(']'),
                    });
                    current_pointer += 1;
                }
                Some(',') => {
                    tokens.push(Token {
                        token_type: TokenType::Comma,
                        value: String::from(','),
                    });
                    current_pointer += 1;
                }
                Some(':') => {
                    tokens.push(Token {
                        token_type: TokenType::Colon,
                        value: String::from(':'),
                    });
                    current_pointer += 1;
                }
                Some('t') => {
                    let mut value = String::new();
                    let mut i = current_pointer;

                    let substring = &json[current_pointer..current_pointer + 4];
                    if substring.starts_with("true") {
                        i += 4;
                        value = String::from("true");
                    }
                    current_pointer = i;
                    tokens.push(Token {
                        token_type: TokenType::True,
                        value,
                    })
                }
                Some('f') => {
                    let mut value = String::new();
                    let mut i = current_pointer;

                    let substring = &json[current_pointer..current_pointer + 5];
                    if substring.starts_with("false") {
                        i += 5;
                        value = String::from("false");
                    }
                    current_pointer = i;
                    tokens.push(Token {
                        token_type: TokenType::False,
                        value,
                    })
                }
                Some('"') => {
                    let mut value = String::new();
                    let mut i = current_pointer;

                    while json.chars().nth(i) != Some('"') {
                        value.push(json.chars().nth(i).expect("Index out of bound"));
                        i += 1;
                    }
                    tokens.push(Token {
                        token_type: TokenType::String,
                        value,
                    });
                    current_pointer = i + 1;
                }
                Some(' ') => current_pointer += 1,
                Some('\n') => current_pointer += 1,
                Some('\t') => current_pointer += 1,

                _ => {
                    return Vec::new();
                }
            }
        }
    }
    tokens
}

pub fn parse_value(tokens: &Vec<Token>, current_pointer: &mut usize) -> bool {
    let token = &tokens[*current_pointer];
    match token.token_type {
        TokenType::String => true,
        TokenType::Number => true,
        TokenType::True => true,
        TokenType::False => true,
        TokenType::Null => true,
        TokenType::OpenBrace => parse_object(tokens, current_pointer),
        TokenType::OpenSqBracket => parse_array(tokens, current_pointer),
        _ => false,
    }
}

pub fn parse_object(tokens: &Vec<Token>, current_pointer: &mut usize) -> bool {
    *current_pointer += 1;
    while *current_pointer < tokens.len()
        && tokens[*current_pointer].token_type != TokenType::CloseBrace
    {
        if tokens[*current_pointer].token_type != TokenType::String {
            return false;
        }

        *current_pointer += 1;

        if tokens[*current_pointer].token_type != TokenType::Colon {
            return false;
        }

        *current_pointer += 1;

        if !parse_value(tokens, current_pointer) {
            return false;
        }

        *current_pointer += 1;

        if *current_pointer < tokens.len()
            && tokens[*current_pointer].token_type == TokenType::Comma
            && tokens[*current_pointer + 1].token_type == TokenType::CloseBrace
        {
            return false;
        }

        if *current_pointer < tokens.len()
            && tokens[*current_pointer].token_type == TokenType::Comma
        {
            *current_pointer += 1;
        }
    }

    if *current_pointer < tokens.len()
        && tokens[*current_pointer].token_type != TokenType::CloseBrace
    {
        return false;
    }

    *current_pointer += 1;

    true
}

pub fn parse_array(tokens: &Vec<Token>, current_pointer: &mut usize) -> bool {
    *current_pointer += 1;
    while *current_pointer < tokens.len()
        && tokens[*current_pointer].token_type != TokenType::CloseSqBracket
    {
        if !parse_value(tokens, current_pointer) {
            return false;
        }

        *current_pointer += 1;

        if *current_pointer < tokens.len()
            && tokens[*current_pointer].token_type == TokenType::Comma
            && tokens[*current_pointer + 1].token_type == TokenType::CloseSqBracket
        {
            return false;
        }

        if *current_pointer < tokens.len()
            && tokens[*current_pointer].token_type == TokenType::Comma
        {
            *current_pointer += 1;
        }
    }

    if *current_pointer < tokens.len()
        && tokens[*current_pointer].token_type != TokenType::CloseSqBracket
    {
        return false;
    }

    // If the json consists only array
    if *current_pointer == &tokens.len() - 1 {
        *current_pointer += 1;
    }

    true // If the array is valid
}

pub fn parse(tokens: &Vec<Token>) -> bool {
    if tokens.len() == 0 {
        return false;
    }

    let mut current_pointer = 0;

    let result = parse_value(&tokens, &mut current_pointer);

    result && current_pointer == tokens.len()
}

fn main() {
    println!("Hello, world!");
}
