use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn print_string(&self) -> String;
}

#[derive(Debug)]
pub enum StatementNode {
    Var(VarStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

impl Node for StatementNode {
    fn token_literal(&self) -> String {
        match self {
            Self::Var(var_stmt) => var_stmt.token_literal(),
            Self::Return(ret_stmt) => ret_stmt.token_literal(),
            Self::Expression(expression) => expression.token_literal(),
        }
    }

    fn print_string(&self) -> String {
        match self {
            Self::Var(var_stmt) => var_stmt.print_string(),
            Self::Return(ret_stmt) => ret_stmt.print_string(),
            Self::Expression(expression) => expression.print_string(),
        }
    }
}

#[derive(Debug)]
pub enum ExpressionNode {
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
    pub statements: Vec<StatementNode>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            match &self.statements[0] {
                StatementNode::Var(var_stmt) => var_stmt.token_literal(),
                StatementNode::Return(ret_stmt) => ret_stmt.token_literal(),
                StatementNode::Expression(expression) => expression.token_literal(),
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

#[derive(Debug)]
pub struct VarStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<ExpressionNode>,
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

#[derive(Debug, Default)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Default)]
pub struct ReturnStatement {
    pub token: Token,
    pub ret_value: Option<ExpressionNode>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn print_string(&self) -> String {
        let mut out = String::from("");

        out.push_str(self.token_literal().as_str());
        out.push_str(" ");

        if let Some(ret_value) = &self.ret_value {
            out.push_str(ret_value.print_string().as_str());
        }

        out.push_str(";");
        out
    }
}

#[derive(Debug, Default)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<ExpressionNode>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        if let Some(expression) = &self.expression {
            return expression.print_string();
        }
        String::from("")
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{ExpressionNode, Identifier, Node, Program, StatementNode, VarStatement};
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_print_string() {
        let program = Program {
            statements: vec![StatementNode::Var(VarStatement {
                token: Token {
                    kind: TokenKind::Var,
                    literal: String::from("var"),
                },
                name: Identifier {
                    token: Token {
                        kind: TokenKind::Identifier,
                        literal: String::from("myVar"),
                    },
                    value: String::from("myVar"),
                },
                value: Some(ExpressionNode::IdentifierNode(Identifier {
                    token: Token {
                        kind: TokenKind::Identifier,
                        literal: String::from("anotherVar"),
                    },
                    value: String::from("anotherVar"),
                })),
            })],
        };

        assert_eq!(
            program.print_string(),
            String::from("var myVar = anotherVar;"),
            "print string wrong, got = {}",
            program.print_string()
        );
    }
}
