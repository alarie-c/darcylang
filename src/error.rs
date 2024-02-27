pub mod error {
    use colored::Colorize;

    pub struct DarcyError {
        kind: ErrorKind,
        line: usize,
        offender: String,
        prefix: String,
        suffix: String,
    }

    impl DarcyError {
        pub fn new(kind: ErrorKind, line: &usize, offender: String, line_content: String) -> Self {
            println!("Offender: {}", offender);
            println!("Content: {}", line_content);
            // If offender is empty, set prefix to line content
            if offender.is_empty() {
                return Self {
                    kind,
                    line: *line,
                    offender,
                    prefix: line_content,
                    suffix: String::new(),
                };
            } else {
                // Get index of offending content
                let idx = line_content
                .find(&offender)
                .expect("ERROR DETERMINING LINE CONTENT FOR ERROR REPORTING");
            
                // Take everything before the index
                let prefix = line_content
                    .get(0..idx)
                    .expect("ERROR DETERMINING PREFIX FOR ERROR REPORTING")
                    .to_string();

                // Take everything after the index
                let suffix = line_content
                    .get(idx + offender.len()..line_content.len())
                    .expect("ERROR DETERMINING SUFFIX FOR ERROR REPORTING")
                    .to_string();

                return Self {
                    kind,
                    line: *line,
                    offender,
                    prefix,
                    suffix,
                };
            }
        }

        fn header(&self) {
            println!("--> {:?} at line {}:", self.kind, self.line);
        }

        fn syntax_error(&self) {
            self.header();

            println!(" |  {}:", "offending line".red());
            println!(" |  \t'{}{}{}'", self.prefix, self.offender, self.suffix);
                        
            // Get report prefix length
            let report_prefix = " ".repeat(self.prefix.len() + 1);

            // Get report arrows length
            let report_arrows = "^".repeat(self.offender.len());
            
            let error_message = format!("{}{} {}\n", report_prefix, report_arrows, self.kind.message());
            println!(" |  \t{}", error_message.red());
        }

        fn identifier_error(&self) {
            self.header();

            // If identifier is empty  
            if self.offender.is_empty() {
                println!(" |  {}:", "offending line".red());
                println!(" |  \t'{}'", self.prefix);
                println!(" |  \texpected an identifier here, found nothing");
            } else {
                println!(" |  {}:", "offending line".red());
                println!(" |  \t'{}{}{}'", self.prefix, self.offender, self.suffix);
                
                // Get report prefix length
                let report_prefix = " ".repeat(self.prefix.len() + 1);
    
                // Get report arrows length
                let report_arrows = "^".repeat(self.offender.len());
    
                let error_message = format!("{}{} {}\n", report_prefix, report_arrows, self.kind.message());
                println!(" |  \t{}", error_message.red());
            }
        }

        pub fn report(&self) {
            match self.kind {
                ErrorKind::IdentifierError => self.identifier_error(),
                ErrorKind::SyntaxError => self.syntax_error(),
                _ => {},
            }
        }
    }
    
    
    #[derive(Debug, PartialEq, Eq)]
    pub enum ErrorKind {
        IdentifierError, 
        SyntaxError,
        OverflowError,
    }

    impl ErrorKind {
        fn message(&self) -> String {
            match self {
                Self::IdentifierError => {
                    return format!("this identifier does not exist in current scope or this is an invalid keyword");
                },
                Self::SyntaxError => {
                    return format!("this symbol is unidentified syntax or is used incorrectly");
                }
                _ => {
                    return format!("");
                },
            }
        }
    }
}