use crate::ast::{Identifier, Program, StatementNode, VarStatement};
use crate::lexer2::Lexer;
use crate::token::{Token, TokenKind};

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Default::default(),
            peek_token: Default::default(),
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program { statements: vec![] };

        while self.current_token.kind != TokenKind::EOF {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<StatementNode> {
        match self.current_token.kind {
            TokenKind::Var => self.parse_var_statement(),
            _ => None,
        }
    }

    fn parse_var_statement(&mut self) -> Option<StatementNode> {
        let mut stmt = VarStatement {
            token: self.current_token.clone(),
            name: Default::default(),
            value: Default::default()
        };

        return if !self.expect_peek(TokenKind::Identifier) {
            None
        } else {
            stmt.name = Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone()
            };

            if !self.expect_peek(TokenKind::Assign) {
                None
            } else {
                self.next_token();
                while self.current_token_is(TokenKind::SemiColon) {
                    self.next_token();
                }
                Some(StatementNode::Var(stmt))
            }
        };
    }

    fn expect_peek(&mut self, token_kind: TokenKind) -> bool {
        if self.peek_token_is(token_kind) {
            self.next_token();
            return true;
        }
        false
    }

    fn peek_token_is(&self, token_kind: TokenKind) -> bool {
        self.peek_token.kind == token_kind
    }

    fn current_token_is(&self, token_kind: TokenKind) -> bool {
        self.current_token.kind == token_kind
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{Node, StatementNode};
    use crate::lexer2::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_var_statements() {
        let input = r#"
            var x = 4;
            var y = 10;
            var foobar = 83838;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        match program {
            Some(program) => {
                assert_eq!(
                    program.statements.len(),
                    3,
                    "statements does not contain 3 statements. got = {}",
                    program.statements.len()
                );

                let expected = vec!["x", "y", "foobar"];

                for (idx, exp) in expected.into_iter().enumerate() {
                    let stmt = &program.statements[idx];
                    test_var_statement(stmt, exp);
                }
            }
            None => panic!("parse program should not be none"),
        }
    }

    fn test_var_statement(stmt: &StatementNode, expected: &str) {
        assert_eq!(
            stmt.token_literal(),
            "var",
            "token literal not `var`. got={}",
            stmt.token_literal()
        );

        match stmt {
            StatementNode::Var(var_stmt) => {
                assert_eq!(
                    var_stmt.name.value, expected,
                    "VarStatement name value not {}. got={}",
                    expected, var_stmt.name.value
                );
                assert_eq!(
                    var_stmt.name.token_literal(),
                    expected,
                    "VarStatement name value not {}. got={}",
                    expected,
                    var_stmt.name.token_literal()
                );
            }
            other => panic!("stmt is not VarStatement. got={:?}", other),
        }
    }
}
