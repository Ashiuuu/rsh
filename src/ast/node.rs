enum AstNode {
    Conditional(ConditionalNode),
}

impl AstNode {
    pub fn execute(&self) -> i64 {
        match self {
            Self::Conditional(n) => n.execute(),
        }
    }
}

struct ConditionalNode {
    condition: Box<AstNode>,
    body: Box<AstNode>,
    fallback: Option<Box<AstNode>>,
}

impl ConditionalNode {
    pub fn execute(&self) -> i64 {
        if self.condition.as_ref().execute() != 0 {
            return self.body.execute();
        }
        self.fallback.as_ref().map(|f| f.execute()).unwrap_or(0)
    }
}
