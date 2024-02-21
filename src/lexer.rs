pub mod tokens {

    #[derive(Debug, PartialEq, Eq)]
    pub enum TokenKind {
        // Grouping Operators
        RPar,
        LPar,
        RBrac,
        LBrac,
        RCurl,
        LCurl,

        // Symbols
        Ampersand,
        Dot,
        Comma,
        Semicolon,
        Colon,
        Tilde,
        SlashSlash,
        Bar,

        // Operators
        Slash,
        Plus,
        Minus,
        PlusEqual,
        MinusEqual,
        Percent,
        Carat,
        Star,

        // Logical Operators
        MoreThan,
        MoreEqual,
        LessThan,
        LessEqual,
        Bang,
        BangEqual,
        Equal,
        EqualEqual,
        And,
        Or,

        // Reserved Words
        Set,
        Let,
        Const,
        While,
        If,
        Else,
        Elif,
        For,
        Func,

        // Other
        End, Empty,
    }

    #[derive(Debug)]
    pub struct Token {
        pub lex: String,
        pub kind: TokenKind,
        pub line: usize,
    }

    impl Token {
        // Create a new token from arguments
        pub fn new(kind: TokenKind, lex: &str, line: usize) -> Self {
            Self {
                kind,
                lex: lex.to_string(),
                line,
            }
        }

        // Return end of file token
        pub fn end(line: usize) -> Self {
            Self {
                kind: TokenKind::End,
                lex: "<END OF FILE>".to_string(),
                line,
            }
        }
    }
}

pub mod lexer {
    use std::iter::Peekable;
    //use crate::error::error::{DarcyError, ErrorKind};
    use super::tokens::{Token, TokenKind};

    // Lexer struct contains data to tokenize file
    pub struct Lexer<Iter: Iterator<Item = char>> {
        pub chars: Peekable<Iter>,
        pub tokens: Vec<Token>,
        pub line: usize,
        pub current: char,
    }

    impl<Iter: Iterator<Item = char>> Lexer<Iter> {

        // Creat a new lexer istance storing iter and neccesary variables
        pub fn new(chars: Peekable<Iter>) -> Self {
            Self {
                chars,
                tokens: Vec::new(),
                line: 1 as usize,
                current: ' ',
            }
        }

        fn advance(&mut self) -> bool {
            if let Some(character) = self.chars.next() {
                self.current = character;
                return true;
            } else {
                self.tokens.push(Token::end(self.line));
                return false;
            }
        }

        fn match_symbols(&mut self) -> Option<Token> {
            match self.current {
                // Grouping Symbols
                '(' => Some(Token::new(TokenKind::LPar, "(", self.line)),
                ')' => Some(Token::new(TokenKind::RPar, ")", self.line)),
                '[' => Some(Token::new(TokenKind::LBrac, "(", self.line)),
                ']' => Some(Token::new(TokenKind::RBrac, "(", self.line)),
                '{' => Some(Token::new(TokenKind::LCurl, "(", self.line)),
                '}' => Some(Token::new(TokenKind::RCurl, "(", self.line)),

                // Other Symbols
                '.' => Some(Token::new(TokenKind::Dot, ".", self.line)),
                ',' => Some(Token::new(TokenKind::Comma, ",", self.line)),
                ':' => Some(Token::new(TokenKind::Colon, ":", self.line)),
                ';' => Some(Token::new(TokenKind::Semicolon, ";", self.line)),
                '*' => Some(Token::new(TokenKind::Star, "*", self.line)),

                // Misc
                '\n' => {
                    self.line += 1;
                    Some(Token::new(TokenKind::Empty, " ", self.line))
                },

                // Logical Operators
                '+' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::PlusEqual, "+=", self.line))
                        } else {
                            Some(Token::new(TokenKind::Plus, "+", self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::Plus, "+", self.line))
                    }
                },
                '-' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::MinusEqual, "-=", self.line))
                        } else {
                            Some(Token::new(TokenKind::Minus, "-", self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::Minus, "-", self.line))
                    }
                },
                '>' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::MoreEqual, ">=", self.line))
                        } else {
                            Some(Token::new(TokenKind::MoreThan, ">", self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::MoreThan, ">", self.line))
                    }
                },
                '<' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::LessEqual, "<=", self.line))
                        } else {
                            Some(Token::new(TokenKind::LessThan, "<", self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::LessThan, "<", self.line))
                    }
                },
                '=' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::EqualEqual, "==", self.line))
                        } else {
                            Some(Token::new(TokenKind::Equal, "=", self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::Equal, "=", self.line))
                    }
                },
                '!' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::BangEqual, "!=", self.line))
                        } else {
                            Some(Token::new(TokenKind::Bang, "!", self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::Bang, "!", self.line))
                    }
                },
                '&' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '&' {
                            self.advance();
                            Some(Token::new(TokenKind::And, "&&", self.line))
                        } else {
                            Some(Token::new(TokenKind::Ampersand, "&", self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::Ampersand, "&", self.line))
                    }
                },
                '|' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '|' {
                            self.advance();
                            return Some(Token::new(TokenKind::Or, "||", self.line));
                        } else {
                            return Some(Token::new(TokenKind::Bar, "|", self.line));
                        }
                    } else {
                        return Some(Token::new(TokenKind::Bar, "|", self.line));
                    }
                },
                _ => {
                    Some(Token::new(TokenKind::Empty, " ", self.line))
                },
            }
        }
        
        // Scan each character of the file
        pub fn scan(&mut self) -> &Vec<Token> {
            // Define variables
            let scanning = true;

            // Advance to first character
            if self.advance() {
                self.current = self.chars.next().unwrap();
            } else {
                return &self.tokens;
            }

            // Match character
            'start: while scanning {
                println!("Current char: {}", self.current);
                // Match character to symbols
                if let Some(t) = self.match_symbols() {
                    self.tokens.push(t);
                    if self.advance() {
                        continue 'start;
                    } else {
                        return &self.tokens;
                    }
                } else {
                    // match something else
                    break 'start;
                }
            }
            
            self.tokens.push(Token::end(self.line));
            &self.tokens
        }
    }
}