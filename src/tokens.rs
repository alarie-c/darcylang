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
        //Tilde,
        //SlashSlash,
        Bar,

        // Operators
        //Slash,
        Plus,
        Minus,
        PlusEqual,
        MinusEqual,
        //Percent,
        //Carat,
        Star,
        RArrow,
        LArrow,

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
        Const,
        End,

        Type(String),

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