pub mod error_kind {
    #[derive(Debug, PartialEq, Eq)]
    pub enum ErrorKind {
        IdentifierError,
        SyntaxError,
    }

    impl ErrorKind {
        pub fn message(&self, content: &str) -> String {
            match self {
                ErrorKind::IdentifierError => {
                    // If content is empty
                    if content.is_empty() {
                        String::from("expected an identifier but found nothing")
                    } else {
                        // Otherwise...
                        String::from("this identifier does not exist in current context or it is an invalid keyword")
                    }
                },
                ErrorKind::SyntaxError => {
                    String::from("Todo")
                },
            }
        }
    }
}

pub mod darcy_error {
    use super::error_kind::ErrorKind;
    use colored::Colorize;

    pub struct DarcyError {
        pub kind: ErrorKind,
        pub line: usize,
        pub content: String,
        pub prefix: String,
        pub suffix: String,
        pub offender: String,
    }

    impl DarcyError {
        // Takes a set of parameters and constructs a new Error instance
        // Logic is employed within the new function to create specialized errors
        pub fn new(content: &str, offender: &str, line: &usize, kind: ErrorKind) -> Self {
            // If offender is empty, set prefix to line content
            if offender.is_empty() {
                Self {
                    kind,
                    line: *line,
                    content: content.to_string(),
                    offender: offender.to_string(),
                    prefix: content.to_string(),
                    suffix: String::new(),
                }
            } else {
                // Get index of offending content
                let idx = content
                .find(&offender)
                .expect("ERROR DETERMINING LINE CONTENT FOR ERROR REPORTING");
            
                // Take everything before the index
                let prefix = content
                    .get(0..idx)
                    .expect("ERROR DETERMINING PREFIX FOR ERROR REPORTING")
                    .to_string();

                // Take everything after the index
                let suffix = content
                    .get(idx + offender.len()..content.len())
                    .expect("ERROR DETERMINING SUFFIX FOR ERROR REPORTING")
                    .to_string();

                Self {
                    kind,
                    line: *line,
                    content: content.to_string(),
                    offender: offender.to_string(),
                    prefix,
                    suffix,
                }
            }
        }

        // Calls functions based on the error kind
        // Prints the error to output
        pub fn report(&self) {
            match self.kind {
                ErrorKind::IdentifierError => identifier_error(&self),
                ErrorKind::SyntaxError => syntax_error(&self),
            }
        }

        // Creates a simple header with the error name and line
        // Associated with an instance of an error
        fn header(&self) {
            println!("--> {:?} at line {}:", self.kind, self.line);
        } 
    }

    fn identifier_error(err: &DarcyError) {
        err.header();

        if err.offender.is_empty() {
            // If identifier is empty  
            println!(" |  {}:", "offending line".red());
            println!(" |  \t'{}'", err.prefix);
            println!(" |  \t{}", err.kind.message(&err.offender));
        } else {
            // Otherwise...
            println!(" |  {}:", "offending line".red());
            println!(" |  \t'{}{}{}'", err.prefix, err.offender, err.suffix);
            
            // Get report prefix length
            let report_prefix = " ".repeat(err.prefix.len() + 1);

            // Get report arrows length
            let report_arrows = "^".repeat(err.offender.len());

            let error_message = format!("{}{} {}\n", report_prefix, report_arrows, err.kind.message(&err.offender));
            println!(" |  \t{}", error_message.red());
        }
    }

    fn syntax_error(err: &DarcyError) {
        todo!();
    }
}