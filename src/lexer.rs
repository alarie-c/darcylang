pub mod tokens {

    #[derive(Debug, PartialEq, Eq)]
    pub enum MatchResult {
        Match(Token),
        None,
    }

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
        Func,
        Matrix,
        Out,
        If,
        Elif,
        Else,
        For,
        While,
        Const,
        End,

        // Other
        EndOfFile, 
        Empty, 
        Newline, 
        StringLiteral, 
        NumberLiteral,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Token {
        pub lex: String,
        pub kind: TokenKind,
        pub line: usize,
    }

    impl Token {
        // Create a new token from arguments
        pub fn new(kind: TokenKind, lex: &str, line: &usize) -> Self {
            Self {
                kind,
                lex: lex.to_string(),
                line: *line,
            }
        }

        // Return end of file token
        pub fn end(line: &usize) -> Self {
            Self {
                kind: TokenKind::EndOfFile,
                lex: "<END OF FILE>".to_string(),
                line: *line,
            }
        }
    }
}

pub mod lexer {
    use std::iter::Peekable;
    use crate::error::error::{DarcyError, ErrorKind};

    //use crate::error::error::{DarcyError, ErrorKind};
    use super::tokens::{MatchResult, Token, TokenKind};

    // Lexer struct contains data to tokenize file
    pub struct Lexer<Iter: Iterator<Item = char>> {
        pub chars: Peekable<Iter>,
        pub tokens: Vec<Token>,
        pub line: usize,
        pub current: char,
        pub lines: Vec<String>,
        pub errors: Vec<DarcyError>,
    }

    impl<Iter: Iterator<Item = char>> Lexer<Iter> {

        // Creat a new lexer istance storing iter and neccesary variables
        pub fn new(chars: Peekable<Iter>, lines: Vec<String>) -> Self {
            Self {
                chars,
                tokens: Vec::new(),
                line: 1 as usize,
                current: ' ',
                lines,
                errors: Vec::new(),
            }
        }

        fn end(&mut self) {
            self.tokens.push(Token::end(&self.line));
        }

        // Throws an error with error.rs file
        // Exits the process
        fn error(&mut self, kind: ErrorKind, offender: &str) {
            // Throw error
            let line_content = self.take_line(self.line);
            match line_content.1 {
                Some(content) => {
                    let err = DarcyError::new(
                        kind,
                        &line_content.0,
                        offender.to_string(),
                        content.to_string(),
                    );
                    self.errors.push(err);

                },
                None => {
                    let err = DarcyError::new(
                        kind,
                        &line_content.0,
                        String::new(),
                        String::new()
                    );
                    self.errors.push(err);
                }
            }
        }

        // Attempts to advance the iterator if possible
        // If not possible, pushes an EOF token and returns None
        fn advance(&mut self) -> bool {
            if let Some(character) = self.chars.next() {
                self.current = character;
                return true;
            } else {
                self.end();
                return false;
            }
        }

        // Attempts to get the content of a line from the lines vec
        // Returns a tuple containing the line # (0) and content (1)
        fn take_line(&mut self, line: usize) -> (usize, Option<&String>) {
            let content = self.lines.get(line - 4);
            match content {
                Some(chars) => {
                    return (line, Some(chars));
                },
                None => (0usize, None),
            }
        }

        // Match a given string to a valid keyword
        // Returns the token of that keyword
        fn match_keyword(&mut self, word: &String) -> Option<Token> {          
            match word.trim() {
                "func" => Some(Token::new(TokenKind::Func, "func", &self.line)),
                "out" => Some(Token::new(TokenKind::Out, "out", &self.line)),
                "if" => Some(Token::new(TokenKind::If, "if", &self.line)),
                "elif" => Some(Token::new(TokenKind::Elif, "elif", &self.line)),
                "else" => Some(Token::new(TokenKind::Else, "else", &self.line)),
                "for" => Some(Token::new(TokenKind::For, "for", &self.line)),
                "matrix" => Some(Token::new(TokenKind::Matrix, "matrix", &self.line)),
                "const" => Some(Token::new(TokenKind::Const, "const", &self.line)),
                "end" => Some(Token::new(TokenKind::End, "end", &self.line)),

                // If word is not a keyword
                _ => None,
            }
        }


        // Match a word to an existing identifier in current scope
        // Returns the token of that identifier
        fn match_id(&mut self, word: &String) -> Option<Token> {
            return None;
        }

        /*
            match_word() returns MatchResult because default Option 'None'
            is used to signify that advance() has reached EOF,

            All helper functions return 'None' back into the scan() method,
            signifying EOF.
        */
        // Tries to match a word to a keyword
        // If not, tries to match to an identifier
        fn match_word(&mut self, word: &String) -> MatchResult {
            // Get keyword match
            let keyword_match = self.match_keyword(word);
            
            // Determine if match found
            match keyword_match {
                // Return keyword if yes
                Some(t) => {
                    return MatchResult::Match(t);
                },
                // Otherwise search for identifier
                None => {
                    if let Some(t) = self.match_id(word) {
                        // Return ID token if found
                        return MatchResult::Match(t);
                    } else {
                        // Otherwise return none
                        return MatchResult::None;
                    }
                }
            }
        }

        // Skips whitespace and advances iterator to a character
        // Returns None if reaches EOF
        fn skip_ws(&mut self) -> Option<()> {
            'wait_for_char: loop {
                if self.advance() {
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

        // Takes all alphanumeric characters
        // Exceptions for _
        fn take_alphanum(&mut self) -> Option<String> {
            let mut buffer = String::new();

            'take_alphanum: loop {
                buffer.push(self.current);

                if self.advance() {
                    if self.current.is_alphanumeric() || self.current == '_' {
                        // Continue of character is alphanumeric or _
                        continue 'take_alphanum;
                    } else {
                        // Return word if character is not
                        if buffer.is_empty() {
                            self.error(ErrorKind::IdentifierError, &buffer);
                            return Some(String::new());
                        } else {
                            return Some(buffer);
                        }
                    }
                } else {    
                    // Signals that advance() returned EOF.
                    return None;
                }
            }
        }

        // Takes number literal
        // Advances the iterator
        fn take_number_literal(&mut self) -> Option<Token> {
            let mut buffer = String::new();

            'literal: loop {
                /*
                    The following allows for valid separation of numbers for readability
                    Example: x = 100_000_000
                */

                // If '_', skip and go to next number
                if self.current == '_' {
                    if self.advance() {
                        continue 'literal;
                    } else {
                        return None;
                    }
                }

                // Push the current to the buffer
                buffer.push(self.current);

                // Go to next character (if possible)
                if self.advance() {
                    // If character is white space
                    if self.current == ' ' {
                        // Drop the carriage return (if present)
                        if buffer.ends_with('\r') {
                            buffer.pop();
                        }

                        // Make token and return
                        let token = Token::new(TokenKind::NumberLiteral, &buffer, &self.line);
                        return Some(token);
                    } else if let Some(t) = self.match_symbols() {
                        // Get exception for dot
                        if t.kind == TokenKind::Dot {
                            // Continue loop (dot gets pushed to buffer)
                            continue 'literal;
                        }

                        // Drop the carriage return (if present)
                        if buffer.ends_with('\r') {
                            buffer.pop();
                        }

                        // Make token and return
                        let token = Token::new(TokenKind::NumberLiteral, &buffer, &self.line);
                        return Some(token);
                    } else {
                        // Continue loop
                        continue 'literal;
                    }
                } else {
                    return None;
                }
            }
        }   

        // Takes string literal
        // Peeks the iterator
        fn take_string_literal(&mut self) -> Option<Token> {
            let mut buffer = String::new();

            // Advance past the quotation mark
            match self.advance() {
                true => {},
                false => return None,
            }

            'literal: loop {
                buffer.push(self.current);

                if let Some(c) = self.chars.peek() {
                    match *c {
                        '"' => {
                            let token = Token::new(TokenKind::StringLiteral, &buffer, &self.line);
                            
                            // Advance past the last quotation
                            match self.advance() {
                                true => {
                                    // Return literal token
                                    return Some(token);
                                },
                                false => {
                                    // Push the literal token
                                    self.tokens.push(token);

                                    // Signal end of source file
                                    return None;
                                },
                            }
                        },
                        _ => {
                            self.advance();
                            continue 'literal;
                        }
                    }
                } else {
                    self.end();
                    return None;
                }
            }
        }

        // Takes alphanumeric character until reaches one that doesn't
        // Attempts to match that buffer to a keyword or identifier
        // Returns the token of the identifier or keyword (if found)
        fn take_alphanum_and_match(&mut self) -> Option<Token> {
            let buffer = self.take_alphanum();

            match buffer {
                Some(string) => {
                    let matched = self.match_word(&string);
                    match matched {
                        // If match then return token
                        MatchResult::Match(t) => return Some(t),
                        MatchResult::None => {
                            self.error(ErrorKind::IdentifierError, &string);
                            return Some(Token::new(TokenKind::Empty, "", &self.line));
                        },
                    }
                },
                None => {
                    // Signals EOF
                    return None;
                }
            }
        }

        fn match_symbols(&mut self) -> Option<Token> {
            match self.current {
                // Grouping Symbols
                '(' => Some(Token::new(TokenKind::LPar, "(", &self.line)),
                ')' => Some(Token::new(TokenKind::RPar, ")", &self.line)),
                '[' => Some(Token::new(TokenKind::LBrac, "[", &self.line)),
                ']' => Some(Token::new(TokenKind::RBrac, "]", &self.line)),
                '{' => Some(Token::new(TokenKind::LCurl, "{", &self.line)),
                '}' => Some(Token::new(TokenKind::RCurl, "}", &self.line)),

                // Other Symbols
                '.' => Some(Token::new(TokenKind::Dot, ".", &self.line)),
                ',' => Some(Token::new(TokenKind::Comma, ",", &self.line)),
                ':' => Some(Token::new(TokenKind::Colon, ":", &self.line)),
                ';' => Some(Token::new(TokenKind::Semicolon, ";", &self.line)),
                '*' => Some(Token::new(TokenKind::Star, "*", &self.line)),

                // Misc
                '\n' => {
                    self.line += 1;
                    Some(Token::new(TokenKind::Newline, "newline", &self.line))
                },
                '\r' => {
                    Some(Token::new(TokenKind::Empty, "empty", &self.line))
                },

                // Logical Operators
                '+' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::PlusEqual, "+=", &self.line))
                        } else {
                            Some(Token::new(TokenKind::Plus, "+", &self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::Plus, "+", &self.line))
                    }
                },
                '-' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::MinusEqual, "-=", &self.line))
                        } else {
                            Some(Token::new(TokenKind::Minus, "-", &self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::Minus, "-", &self.line))
                    }
                },
                '>' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::MoreEqual, ">=", &self.line))
                        } else {
                            Some(Token::new(TokenKind::MoreThan, ">", &self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::MoreThan, ">", &self.line))
                    }
                },
                '<' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::LessEqual, "<=", &self.line))
                        } else {
                            Some(Token::new(TokenKind::LessThan, "<", &self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::LessThan, "<", &self.line))
                    }
                },
                '=' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::EqualEqual, "==", &self.line))
                        } else {
                            Some(Token::new(TokenKind::Equal, "=", &self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::Equal, "=", &self.line))
                    }
                },
                '!' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '=' {
                            self.advance();
                            Some(Token::new(TokenKind::BangEqual, "!=", &self.line))
                        } else {
                            Some(Token::new(TokenKind::Bang, "!", &self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::Bang, "!", &self.line))
                    }
                },
                '&' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '&' {
                            self.advance();
                            Some(Token::new(TokenKind::And, "&&", &self.line))
                        } else {
                            Some(Token::new(TokenKind::Ampersand, "&", &self.line))
                        }
                    } else {
                        Some(Token::new(TokenKind::Ampersand, "&", &self.line))
                    }
                },
                '|' => {
                    if let Some(c) = self.chars.peek() {
                        if *c == '|' {
                            self.advance();
                            return Some(Token::new(TokenKind::Or, "||", &self.line));
                        } else {
                            return Some(Token::new(TokenKind::Bar, "|", &self.line));
                        }
                    } else {
                        return Some(Token::new(TokenKind::Bar, "|", &self.line));
                    }
                },

                // String literal
                '"' => {
                    if let Some(t) = self.take_string_literal() {
                        return Some(t);
                    } else {
                        return None;
                    }
                }
                _ => {
                    None
                },
            }
        }
        
        // Scan each character of the file
        pub fn scan(&mut self) -> (&Vec<Token>, &Vec<DarcyError>) {
            // Define variables
            let scanning = true;

            // Match character
            'start: while scanning {
                println!("Current char: `{}`", self.current);

                // Skip token if whitespace
                if self.current == ' ' || self.current == '\t' || self.current == '\r' {
                    if self.advance() {
                        continue 'start;
                    } else {
                        return (&self.tokens, &self.errors); 
                    }
                }

                // Match character to symbols
                if let Some(t) = self.match_symbols() {
                    self.tokens.push(t);
                    if self.advance() {
                        continue 'start;
                    } else {
                        return (&self.tokens, &self.errors);
                    }
                } else {
                    // Check if character could be a number literal
                    /*
                        This if statement repeats 'start if literal found,
                        no need for else statement (avoids nesting)
                    */

                    if self.current.is_numeric() {
                        println!("This is numeric");
                        // Take number as a literal and parse
                        if let Some(t) = self.take_number_literal() {
                            self.tokens.push(t);
                            continue 'start;
                        } else {
                            // If none returned
                            self.end();
                            return (&self.tokens, &self.errors);
                        }
                    }

                    // Match character to identifier/keyword
                    if let Some(t) = self.take_alphanum_and_match() {

                        // Match returned 
                        match t.kind {
                            // End: stop tokenizing
                            TokenKind::EndOfFile => {
                                self.tokens.push(t);
                                return (&self.tokens, &self.errors);
                            },

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
            
            self.tokens.push(Token::end(&self.line));
            (&self.tokens, &self.errors)
        }
    }
}