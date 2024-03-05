pub mod errors {
    // Different error types take different values.
    // Every error type takes a usize for the line number.
    pub enum Errors {
        IdentifierError(usize, String),
        SyntaxError(usize, String),
        ArgumentError(usize, u8, u8),
    }

    impl Errors {
        fn report_header(kind: &str, line: &usize) {
            println!("--> {kind} at line {line}:");
        }

        fn argument_error(&self, source: Vec<String>, line: &usize, params: u8, args: u8) {
            let content = source[line - 1];
            let open_par = content.find("(").unwrap();
            let close_par = content.find(")").unwrap();
            
            let prefix = content[0..open_par];
            let suffix = content[close_par..content.len()];
            let arguments = content[open_par..close_par];

            let report_line = format!("{prefix}{arguments}{suffix}");
            let report_spaces = " ".repeat(prefix.len() + 1);
            let report_arrows = "^".repeat(arguments.len());
            let report_msg = format!("{params} parameters were specific but {args} arguments were passed.");

            // Print the error
            Self::report_header("ArgumentError", line);
            println!(" | offending line:");
            println!(" | {report_line}");
            println!(" | {report_spaces}{report_arrows} {report_msg}");
        }

        pub fn report(&self, source: Vec<String>) {
            // Determine error type
            match self {
                Errors::IdentifierError(line, offender) => {
                    todo!();
                },
                Errors::SyntaxError(line, offender) => {
                    todo!();
                },
                Errors::ArgumentError(line, params, args) => {
                    // Example:
                    // my_function(5, 10, 2)
                    //             ^^^^^^^^ 2 parameters specified, but 3 arguments were given
                    
                    self.argument_error(source, line, params, args);
                },
            }
        }
    }
}