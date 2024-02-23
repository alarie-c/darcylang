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
        Out,

        // Other
        End, Empty, Newline,
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
    use std::{error::Error, iter::Peekable};
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

        fn end(&mut self) {
            self.tokens.push(Token::end(self.line));
        }

        fn advance(&mut self) -> Option<()> {
            if let Some(character) = self.chars.next() {
                self.current = character;
                return Some(());
            } else {
                self.tokens.push(Token::end(self.line));
                return None;
            }
        }

        fn match_keyword(&mut self, word: &String) -> Option<Token> {
            println!("{word}");
            
            match word.trim() {
                // Match word to a keyword
                "let" => {
                    // Return keyword token
                    return Some(Token::new(TokenKind::Let, "let", self.line));
                },
                "set" => {
                    // Return keyword token
                    return Some(Token::new(TokenKind::Set, "set", self.line));
                },
                "const" => {
                    // Return keyword token
                    return Some(Token::new(TokenKind::Const, "const", self.line));
                },
                "while" => {
                    // Return keyword token
                    return Some(Token::new(TokenKind::While, "while", self.line));
                },
                "if" => {
                    // Return keyword token
                    return Some(Token::new(TokenKind::If, "if", self.line));
                },
                "else" => {
                    // Return keyword token
                    return Some(Token::new(TokenKind::Else, "else", self.line));
                },
                "elif" => {
                    // Return keyword token
                    return Some(Token::new(TokenKind::Elif, "elif", self.line));
                },
                "for" => {
                    // Return keyword token
                    return Some(Token::new(TokenKind::For, "for", self.line));
                },
                "func" => {
                    // Return keyword token
                    return Some(Token::new(TokenKind::For, "for", self.line));
                },
                "out" => {
                    // Return keyword token
                    return Some(Token::new(TokenKind::For, "for", self.line));
                },

                // If word is not a keyword
                _ => {
                    return None;
                }
            }
        }

        fn match_id(&mut self, word: &String) -> Option<Token> {
            todo!();
        }

        fn match_word(&mut self, word: String) -> Option<Token> {
            // Get keyword match
            let keyword_match = self.match_keyword(&word);
            
            // Determine if match found
            match keyword_match {
                // Return keyword if yes
                Some(t) => {
                    return Some(t);
                },
                // Otherwise search for identifier
                None => {
                    if let Some(t) = self.match_id(&word) {
                        // Return ID token if found
                        return Some(t);
                    } else {
                        // Otherwise return none
                        return None;
                    }
                }
            }
        }

        fn take_until_char(&mut self) -> Option<()> {

            'wait_for_char: loop {
                if let Some(_) = self.advance() {
                    if self.current == ' ' {
                        continue 'wait_for_char;
                    } else {
                        return Some(());
                    }
                } else {
                    return None;
                }
            }
        }

        fn take_word(&mut self) -> Option<String> {
            let mut id = String::new();

            'word: loop {
                id.push(self.current);
                if let Some(_) = self.advance() {
                    if self.current == ' ' {
                        if id.is_empty() || id == " " {
                            // TODO: darcy error handling
                            eprintln!("No identifier given for let statement error!");
                            std::process::exit(1);
                        } else {
                            return Some(id);
                        }
                    } else if let Some(t) = self.match_symbols() {
                        // Push symbol token
                        self.tokens.push(t);

                        // Return identifier
                        if id.is_empty() || id == " " {
                            // TODO: darcy error handling
                            eprintln!("No identifier given for let statement error!");
                            std::process::exit(1);
                        } else {
                            return Some(id);
                        }
                    }
                } else {
                    return None;
                }
            }
        }

        fn take_word_and_match(&mut self) -> Option<Token> {
            let mut id = String::new();
            
            // Take all characters until we reach whitespace or a symbol
            'word: loop {
                id.push(self.current);
                println!("{id}");
                
                if let Some(_) = self.advance() {
                    if self.current == ' ' {
                        let matched = self.match_word(id);
                        match matched {
                            // If match_word() returns a valid token
                            Some(t) => {
                                // Return recieved token
                                return Some(t);
                            },

                            // If match_word() returns None
                            None => {
                                // TODO: add darcy error handling here!
                                eprintln!("Unknown ID error");
                                todo!();
                            },
                        }
                    } else if let Some(t) = self.match_symbols() {
                        let matched = self.match_word(id);
                        match matched {
                            // If match_word() returns a valid token
                            Some(matched_t) => {
                                // Push the consumed symbol token
                                self.tokens.push(t);

                                // Return recieved token
                                return Some(matched_t);
                            },

                            // If match_word() returns None
                            None => {
                                // TODO: add darcy error handling here!
                                eprintln!("Unknown ID error");
                                todo!();
                            },
                        }
                    } else {
                        // Repeat
                        continue 'word;
                    }
                } else {
                    return Some(
                        Token::new(TokenKind::End, "<END OF FILE>", self.line)
                    );
                }
            }
        }

        fn match_symbols(&mut self) -> Option<Token> {
            match self.current {
                // Grouping Symbols
                '(' => Some(Token::new(TokenKind::LPar, "(", self.line)),
                ')' => Some(Token::new(TokenKind::RPar, ")", self.line)),
                '[' => Some(Token::new(TokenKind::LBrac, "[", self.line)),
                ']' => Some(Token::new(TokenKind::RBrac, "]", self.line)),
                '{' => Some(Token::new(TokenKind::LCurl, "{", self.line)),
                '}' => Some(Token::new(TokenKind::RCurl, "}", self.line)),

                // Other Symbols
                '.' => Some(Token::new(TokenKind::Dot, ".", self.line)),
                ',' => Some(Token::new(TokenKind::Comma, ",", self.line)),
                ':' => Some(Token::new(TokenKind::Colon, ":", self.line)),
                ';' => Some(Token::new(TokenKind::Semicolon, ";", self.line)),
                '*' => Some(Token::new(TokenKind::Star, "*", self.line)),

                // Misc
                '\n' => {
                    self.line += 1;
                    Some(Token::new(TokenKind::Newline, " ", self.line))
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
                    None
                },
            }
        }
        
        // Scan each character of the file
        pub fn scan(&mut self) -> &Vec<Token> {
            // Define variables
            let scanning = true;

            // Advance to first character
            // if let Some(_) = self.advance() {
            //     self.current = self.chars.next().unwrap();
            // } else {
            //     return &self.tokens;
            // }

            // Match character
            'start: while scanning {
                println!("Current char: {}", self.current);

                // Skip token if whitespace
                if self.current == ' ' || self.current == '\t' || self.current == '\r' {
                    if let Some(_) = self.advance() {
                        continue 'start;
                    } else {
                        return &self.tokens;
                    }
                }

                // Match character to symbols
                if let Some(t) = self.match_symbols() {
                    self.tokens.push(t);
                    if let Some(_) = self.advance() {
                        continue 'start;
                    } else {
                        return &self.tokens;
                    }
                } else {
                    // Match character to identifier/keyword

                    if let Some(mut t) = self.take_word_and_match() {

                        // Match returned 
                        match t.kind {
                            // End: stop tokenizing
                            TokenKind::End => {
                                self.tokens.push(t);
                                return &self.tokens;
                            },

                            // Let: get var name as literal 
                            TokenKind::Let => {
                                
                                // Get identifier
                                if let Some(_) = self.take_until_char() {
                                    if let Some(id) = self.take_word() {
                                        t.lex = id;
                                        self.tokens.push(t);
                                    } else {
                                        self.end();
                                        return &self.tokens;
                                    }
                                } else {
                                    self.end();
                                    return &self.tokens;
                                }
                            },
                            
                            // Set: get var from context
                            TokenKind::Set => {
                                if let Some(_) = self.take_until_char() {
                                    todo!();
                                }
                            }


                            _ => {
                                self.tokens.push(t);
                                continue 'start;
                            },
                        }
                    } else {
                        continue 'start;
                    }
                }
            }
            
            self.tokens.push(Token::end(self.line));
            &self.tokens
        }
    }
}