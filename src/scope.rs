pub mod values {
    use super::scope::Environment;

    #[derive(Debug, PartialEq, Clone)]
    pub enum ValueKind {
        // Integers
        IntUnsigned(u32),
        IntSigned(i32),
        // IntLongUnsign(u64),
        // IntLongSigned(i64),

        // Numbers
        Number(f32),
        // NumberLong(f64),
    }

    // Value struct stores the kind, data, and environment a value belongs to
    // Values can be linked to identifiers via HashMap
    #[derive(Debug)]
    pub struct Value {
        kind: ValueKind,
        env: Environment,
    }

    impl Value {
        // Constructs a new value with the kind specified
        pub fn new(kind: ValueKind, env: Environment) -> Self {
            Self {
                kind,
                env,
            }
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
        values: HashMap<String, Value>,
        parent: usize,
        children: Vec<usize>,
        id: usize,
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