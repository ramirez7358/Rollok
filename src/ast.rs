trait Node {
    fn token_literal(&self) -> String;
    fn print_string(&self) -> String;
}

enum StatementNode {
    Let,
}

impl Node for StatementNode {
    fn token_literal(&self) -> String {
        match self {
            Self::Let => todo!()
        }
    }

    fn print_string(&self) -> String {
        match self {
            Self::Let => todo!()
        }
    }
}

enum ExpressionNode {
    Identifier,
}

impl Node for ExpressionNode {
    fn token_literal(&self) -> String {
        match self {
            Self::Identifier => todo!()
        }
    }

    fn print_string(&self) -> String {
        match self {
            Self::Identifier => todo!()
        }
    }
}

struct Program {
    statements: Vec<StatementNode>
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            match &self.statements[0] {
                StatementNode::Let => todo!()
            }
        } else {
            String::from("")
        }
    }

    fn print_string(&self) -> String {
        let mut output = String::from("");

        for stmt in self.statements.as_slice() {
            output.push_str(stmt.print_string().as_str());
        }
        output
    }
}