pub mod nodes {
    pub enum OperatorKind {
        Plus,
        Minus,
    }

    pub enum NodeKind {
        // Unary expression
        // Ex: -1
        UnaryExpr {
            op: OperatorKind,
            child: Box<NodeKind>,
        },

        // Binary expression
        // Ex: 2 + 2
        BinaryExpr {
            op: OperatorKind,
            left: Box<NodeKind>,
            right: Box<NodeKind>,
        },
    }
}