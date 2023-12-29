use crate::ast::{Program, StatementNode};
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

    fn parse_statement(&self) -> Option<StatementNode> {
        match self.current_token.kind {
            TokenKind::Var => self.parse_var_statement(),
            _ => None,
        }
    }

    fn parse_var_statement(&self) -> Option<StatementNode> {
        None
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
