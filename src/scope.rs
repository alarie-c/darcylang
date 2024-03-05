pub mod values {
    use super::scope::Environment;

<<<<<<< HEAD
    #[derive(Debug, PartialEq, Clone)]
    pub enum ValueKind {
=======
    #[derive(Debug, PartialEq)]
    pub enum Type {
>>>>>>> bd1a31444c128380d505fb63bcef4d74f948947a
        // Integers
        Integer(i32),

        // Numbers
        Number(f32),
        
        // Other
        Str(String),
    }

    // Value struct stores the kind, data, and environment a value belongs to
    // Values can be linked to identifiers via HashMap
    #[derive(Debug)]
    pub struct Value {
        typ: Type,
        env: u8,
    }

    impl Value {
        // Constructs a new value with the kind specified
        pub fn num_or_int(val: String, env: u8) -> Self {
            // Determine value type
            if val.contains(".") {
                // Parse floating point literals
                let num: f32 = match val.parse() {
                    Ok(v) => v,
                    Err(_) => 0.0 // TODO: Error handle here
                };

                // Return value with Number kind
                Self {
                    typ: Type::Number(num),
                    env,
                }
            } else {
                // Parse integer literals
                let num: i32 = match val.parse() {
                    Ok(v) => v,
                    Err(_) => 0 // TODO: Error handle here
                };

                // Return self with Integer kind
                Self {
                    typ: Type::Integer(num),
                    env,
                }
            }
        }

        // Creates a new value with string type
        // Returns the value instance
        pub fn str(val: String, env: u8) -> Self {
            Self {
                typ: Type::Str(val),
                env,
            }
        }
        
        // Creates a new value with the type specified in parameteres
        // Returns the value instance
        pub fn from(typ: &str, val: String, env: u8) -> Self {
            match typ {
                "int" => {
                    Self {
                        typ: Type::Integer(Self::parse_int(val)),
                        env,
                    }
                },
                "num" => {
                    Self {
                        typ: Type::Number(Self::parse_float(val)),
                        env,
                    }
                },
                _ => panic!("KIND SPECIFIER `{}` NOT VALID!", typ)
            }
        }

        // Helper function that parses integers
        fn parse_int(val: String) -> i32 {
            // Parse integer literals
            let num: i32 = match val.parse() {
                Ok(v) => v,
                Err(_) => 0 // TODO: Error handle here
            };

            num
        }

        // Helper function that parses floats
        fn parse_float(val: String) -> f32 {
            // Parse floating point literals
            let num: f32 = match val.parse() {
                Ok(v) => v,
                Err(_) => 0.0 // TODO: Error handle here
            };

            num
        }
    }
}

pub mod scope {
    use std::collections::HashMap;
    use super::values::Value;
    
    pub struct GlobalEnvironment {
        pub values: HashMap<String, Value>,
        pub children: Vec<Environment>,
    }

    #[derive(Debug)]
    pub struct Environment {
        // Values takes keys in the form of Strings and matches them with Value structs
        pub values: HashMap<String, Value>,
        pub parent: usize,
        pub children: Vec<usize>,
        pub id: usize,
    }

    impl Environment {
        // Takes an instance of GlobalEnvironment and any parent Environments (0 for GE)
        // Returns the ID of the environment to be used as a pointer
        pub fn new(g: &mut GlobalEnvironment, parent: usize) -> usize {
            // Construct a new environment
            let environment = Self { 
                values: HashMap::new(),
                parent,
                children: Vec::new(),
                id: g.children.len() + 1,
            };

            // Get the ID of the environment
            let return_id = environment.id;

            // Push the environment to the GlobalEnvironment
            g.children.push(environment);
            
            // Return the ID
            return_id
        }
    }
}