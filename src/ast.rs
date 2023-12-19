use crate::token::Token;

trait Node {
    fn token_literal(&self) -> String;
    fn print_string(&self) -> String;
}

enum StatementNode {
    Var(VarStatement),
}

impl Node for StatementNode {
    fn token_literal(&self) -> String {
        match self {
            Self::Var(var_stmt) => var_stmt.token_literal(),
        }
    }

    fn print_string(&self) -> String {
        match self {
            Self::Var(var_stmt) => var_stmt.print_string(),
        }
    }
}

enum ExpressionNode {
    IdentifierNode(Identifier),
}

impl Node for ExpressionNode {
    fn token_literal(&self) -> String {
        match self {
            Self::IdentifierNode(identifier) => identifier.token_literal(),
        }
    }

    fn print_string(&self) -> String {
        match self {
            Self::IdentifierNode(identifier) => identifier.print_string(),
        }
    }
}

pub struct Program {
    statements: Vec<StatementNode>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            match &self.statements[0] {
                StatementNode::Var(var_stmt) => var_stmt.token_literal(),
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

struct VarStatement {
    token: Token,
    name: Identifier,
    value: Option<ExpressionNode>,
}

impl Node for VarStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        let mut out = String::from("");

        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.print_string());
        out.push_str(" = ");

        if let Some(value) = &self.value {
            out.push_str(&value.print_string());
        };
        out.push_str(";");

        out
    }
}

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        self.value.clone()
    }
}
