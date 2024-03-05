pub mod nodes {
    use crate::scope::values::{Value, ValueKind};

    #[derive(Debug, Clone)]
    pub enum Node {
        Binary(BinaryNode),
        Unary(UnaryNode),
        Literal(LiteralNode),
        Ignore,
    }

    #[derive(Debug, Clone)]
    pub enum UnaryOp {
        //increment,
        //decrement,
        Negative,
    }

    #[derive(Debug, Clone)]
    pub enum BinaryOp {
        Plus,
        Minus,
        Multiply,
        Divide,
    }

    #[derive(Debug, Clone)]
    pub enum Literal {
        Interger(i32),
        Float(f64),
        String(String),
    }

    #[derive(Debug, Clone)]
    pub struct BinaryNode {
        pub left: Box<Node>,
        pub right: Box<Node>,
        pub op: BinaryOp,
    }

    #[derive(Debug, Clone)]
    pub struct LiteralNode {
        pub literal: Literal
    }
    
    #[derive(Debug, Clone)]
    pub struct UnaryNode {
        pub right: Box<Node>,
        pub op: UnaryOp,
    }
}

pub mod ast {
    use crate::{ast::nodes::{Literal, LiteralNode}, tokens::tokens::{Token, TokenKind}};

    use super::nodes::{BinaryNode, BinaryOp, Node};

    pub struct Ast {
        idx: usize,
        pub tokens: Vec<Token>,
        pub nodes: Vec<Node>,
    }

    impl Ast {
        fn match_token(&mut self, token: &Token, idx: usize) -> Node {
            match token.kind {
                // Float and integer literals
                TokenKind::NumberLiteral(lexeme) => {
                    // Check if token is a float
                    if lexeme.contains(".") {
                        let parsed = lexeme.parse::<f64>();
                        match parsed {
                            Ok(_) => {}
                            Err(_) => {
                                panic!("Error parsing number literal lexeme to float64");
                                // TODO: Error handle here
                            }
                        }

                        let literal = Literal::Float(parsed.unwrap());
                        let literal_node = LiteralNode { literal };
                        
                        Node::Literal(literal_node)
                    } else {
                        // If token is an integer
                        let parsed = lexeme.parse::<i32>();
                        match parsed {
                            Ok(_) => {},
                            Err(_) => {
                                panic!("Error parsing number literal lexeme to signed int32")
                                // TOOD: Error handle here
                            }
                        }

                        let literal = Literal::Interger(parsed.unwrap());
                        let literal_node = LiteralNode { literal };
                        
                        Node::Literal(literal_node)
                    }
                },
                TokenKind::Plus => {
                    // Get the left node
                    let left: Box<Node>;
                    if let Some(last_node) = self.nodes.last() {
                        let n = last_node.clone();
                        left = Box::new(n);
                    } else {
                        panic!("Error determining last token in nodes!")
                        // TODO: Error handle here
                    }

                    // Get the right node
                    let right: Box<Node>;
                    let next_token = self.tokens[self.idx + 1].clone();
                    let right_node = self.match_token(&next_token, self.idx);
                    let n = right_node.clone();
                    right = Box::new(n);

                    // Construct the binary node
                    let op = BinaryOp::Plus;
                    let binary = BinaryNode { left, right, op, };
                    
                    Node::Binary(binary)
                },
                _ => Node::Ignore
            }
        }

        // Create a new AST struct with the tokens and empty node vec
        pub fn new(tokens: Vec<Token>) -> Self {
            Self {
                idx: 0 as usize,
                tokens,
                nodes: Vec::new(),
            }
        }

        // Take the tokens from AST.Tokens and construct the AST
        pub fn build(&mut self) {
            // Iterate over every token
            let len = self.tokens.len();

            let mut nodes = Vec::<Node>::new();

            'tokens: loop {
                while self.idx <= len {
                    let t = &self.tokens[self.idx];
                    let node = self.match_token(t, self.idx);

                    // Push node to nodes
                    nodes.push(node);

                    // Continue to next token
                    self.idx += 1;
                    continue 'tokens;
                }

                println!("Reached the end of the tokens list");
                break 'tokens;
            }
        }

        fn binary(left: Node, right: Node, op: BinaryOp) -> Node {
            let node = BinaryNode {
                left: Box::new(left),
                right: Box::new(right),
                op,
            };

            Node::Binary(node)
        }
    }
}