pub mod tokens {
    //use std::{collections::HashMap};

    #[derive(Debug, Copy, Clone)]
    pub enum Kind {
        // Grouping Operators
        RPar, LPar,
        RBrac, LBrac,
        RCurl, LCurl,

        // Logical Operators
        Plus, Minus,
        PlusEqual,
        MinusEqual,
        Equal,
        EqualEqual,
        Bang,
        BangEqual,
        // MoreThan,
        // MoreEqual,
        // LessThan,
        // LessEqual,
        // And, Or,

        // Misc Characters
        Dot, Comma,
        Semicolon,
        Colon, Star,

        // Keywords
        Let, Set, Const, While,


        End,
    }
    
    #[derive(Debug)]
    pub struct Token {
        pub kind: Kind,
        pub lexeme: String,
        pub line: usize,
    }

    impl Token {
        pub fn new(kind: Kind, lexeme: &str, line: usize) -> Self {
            Self {
                kind,
                lexeme: lexeme.to_string(),
                line,
            }
        }

        pub fn display(&self) -> String {
            format!("{} - {}", self.line, self.lexeme)
        }
    }

    // pub fn keywords(mut hashmap: HashMap<String, Kind>) -> HashMap<String, Kind> {
    //     hashmap.insert("let".to_string(), Kind::Let);
    //     hashmap.insert("set".to_string(), Kind::Set);
    //     hashmap.insert("const".to_string(), Kind::Const);

    //     hashmap
    // }
}

pub mod lexer {
    //use std::{collections::HashMap};

    use super::tokens::{/*keywords,*/Kind, Token};
    pub struct Lexer {
        //keywords: HashMap<String, Kind>,
    }

    impl Lexer {
        pub fn new() -> Self {
            //let map: HashMap<String, Kind> = HashMap::new();
            //let keywords = keywords(map);
            
            Self {
                //keywords,
            }
        }
        pub fn scan(&self, input: String) -> Vec<Token> {
            let mut line: usize = 1;
            
            let mut res: Vec<Token> = Vec::new();

            let mut chars = input.chars().peekable();
            'outer: while let Some(c) = chars.next() {
                println!("Current char: {:?}", c);
                match c {
                    
                    // Grouping Operators
                    ')' => res.push(Token::new(Kind::RPar, ")", line)),
                    '(' => res.push(Token::new(Kind::LPar, "(", line)),
                    ']' => res.push(Token::new(Kind::RBrac, "]", line)),
                    '[' => res.push(Token::new(Kind::LBrac, "[", line)),
                    '}' => res.push(Token::new(Kind::RCurl, "}", line)),
                    '{' => res.push(Token::new(Kind::LCurl, "{", line)),

                    // Misc Characters
                    '\n' => line += 1,
                    '\r' => {},

                    '.' => res.push(Token::new(Kind::Dot, ".", line)),
                    ',' => res.push(Token::new(Kind::Comma, ",", line)),
                    ';' => res.push(Token::new(Kind::Semicolon, ";", line)),
                    ':' => res.push(Token::new(Kind::Colon, ":", line)),
                    '*' => res.push(Token::new(Kind::Star, "*", line)),
                    ' ' => {
                        continue 'outer;
                    },

                    // Logcal Operators
                    '!' => {
                        if *chars.peek().unwrap() == '=' {
                            res.push(Token::new(Kind::BangEqual, "!=", line));
                            chars.next();
                        } else {
                            res.push(Token::new(Kind::Bang, "!", line));
                        }
                    },
                    '=' => {
                        if *chars.peek().unwrap() == '=' {
                            res.push(Token::new(Kind::EqualEqual, "==", line));
                            chars.next();
                        } else {
                            res.push(Token::new(Kind::Equal, "=", line));
                        }
                    },
                    '+' => {
                        if *chars.peek().unwrap() == '=' {
                            res.push(Token::new(Kind::PlusEqual, "+=", line));
                            chars.next();
                        } else {
                            res.push(Token::new(Kind::Plus, "+", line));
                        }
                    },
                    '-' => {
                        if *chars.peek().unwrap() == '=' {
                            res.push(Token::new(Kind::MinusEqual, "-=", line));
                            chars.next();
                        } else {
                            res.push(Token::new(Kind::Minus, "-", line));
                        }
                    },
                    '>' => {
                        if *chars.peek().unwrap() == '=' {
                            res.push(Token::new(Kind::MinusEqual, ">=", line));
                            chars.next();
                        } else {
                            res.push(Token::new(Kind::Minus, ">", line));
                        }
                    },
                    '<' => {
                        if *chars.peek().unwrap() == '=' {
                            res.push(Token::new(Kind::MinusEqual, "<=", line));
                            chars.next();
                        } else {
                            res.push(Token::new(Kind::Minus, "<", line));
                        }
                    },
                    _ => {
                        // Get keyword
                        //let mut is_matching = true;
                        println!("Trying to match keyword!");
                        let mut result = String::from(c);

                        'keyword: loop {
                            println!("Entered keyword loop!");
                            let next = *chars.peek().unwrap();
                            if next == '\n' || next == ';' {
                                eprintln!("Unexpected char err");
                                std::process::exit(1);
                            } else if next == ' ' {
                                continue 'outer;
                            } else {
                                result.push(next);
                                println!("{result}");

                                match result.trim() {
                                    "let" => {
                                        println!("Found token let");
                                        res.push(Token::new(Kind::Let, "let", line));
                                        chars.next();
                                        continue 'outer;
                                    },
                                    "set" => {
                                        println!("Found token set");
                                        res.push(Token::new(Kind::Set, "set", line));
                                        chars.next();
                                        continue 'outer;
                                    },
                                    "const" => {
                                        println!("Found token const");
                                        res.push(Token::new(Kind::Const, "const", line));
                                        chars.next();
                                        continue 'outer;
                                    },
                                    "while" => {
                                        println!("Found token while");
                                        res.push(Token::new(Kind::While, "while", line));
                                        chars.next();
                                        continue 'outer;
                                    },
                                    _ => {},
                                }
                                chars.next();
                                continue 'keyword;
                            }
                        }
                    }
                }
            }

            res.push(Token::new(Kind::End, "<END>", line));
            res
        }
    }



}