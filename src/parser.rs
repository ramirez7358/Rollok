use crate::ast::{
    ExpressionNode, ExpressionStatement, Identifier, IntegerLiteral, Program, ReturnStatement,
    StatementNode, VarStatement,
};
use crate::lexer2::Lexer;
use crate::token::{Token, TokenKind};
use std::collections::HashMap;

type PrefixParseFn = fn(parser: &mut Parser) -> Option<ExpressionNode>;
type InfixParseFn = fn(parser: &mut Parser, exp: ExpressionNode) -> Option<ExpressionNode>;

enum PrecedenceLevel {
    Lowest = 0,
    Equals = 1,
    LessGreater = 2,
    Sum = 3,
    Product = 4,
    Prefix = 5,
    Call = 6,
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenKind, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenKind, InfixParseFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Default::default(),
            peek_token: Default::default(),
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        parser.register_prefix(TokenKind::Identifier, Self::parse_identifier);
        parser.register_prefix(TokenKind::Number, Self::parse_integer_literal);

        parser.next_token();
        parser.next_token();
        parser
    }

    fn parse_integer_literal(&mut self) -> Option<ExpressionNode> {
        let mut literal = IntegerLiteral {
            token: self.current_token.clone(),
            value: Default::default(),
        };

        return match self.current_token.literal.parse::<i64>() {
            Ok(value) => {
                literal.value = value;
                Some(ExpressionNode::Integer(literal))
            }
            Err(_) => {
                let msg = format!("could not parse {} as integer", self.current_token.literal);
                self.errors.push(msg);
                None
            }
        };
    }

    fn parse_identifier(&mut self) -> Option<ExpressionNode> {
        Some(ExpressionNode::IdentifierNode(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }))
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program { statements: vec![] };

        while !self.current_token_is(TokenKind::EOF) {
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
            TokenKind::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<StatementNode> {
        let stmt = ExpressionStatement {
            token: self.current_token.clone(),
            expression: self.parse_expression(PrecedenceLevel::Lowest),
        };

        if self.peek_token_is(TokenKind::SemiColon) {
            self.next_token();
        }

        Some(StatementNode::Expression(stmt))
    }

    fn parse_expression(&mut self, precedence: PrecedenceLevel) -> Option<ExpressionNode> {
        let prefix = self.prefix_parse_fns.get(&self.current_token.kind);
        if let Some(prefix_fn) = prefix {
            let left_exp = prefix_fn(self);

            return left_exp;
        }
        None
    }

    fn parse_return_statement(&mut self) -> Option<StatementNode> {
        let stmt = ReturnStatement {
            token: self.current_token.clone(),
            ret_value: Default::default(),
        };
        self.next_token();

        while !self.current_token_is(TokenKind::SemiColon) {
            self.next_token();
        }

        Some(StatementNode::Return(stmt))
    }

    fn parse_var_statement(&mut self) -> Option<StatementNode> {
        let mut stmt = VarStatement {
            token: self.current_token.clone(),
            name: Default::default(),
            value: Default::default(),
        };

        return if !self.expect_peek(TokenKind::Identifier) {
            None
        } else {
            stmt.name = Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
            };

            if !self.expect_peek(TokenKind::Assign) {
                None
            } else {
                self.next_token();
                // TODO: need to parse expression
                while self.peek_token_is(TokenKind::SemiColon) {
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
        self.peek_error(token_kind);
        false
    }

    fn peek_token_is(&self, token_kind: TokenKind) -> bool {
        self.peek_token.kind == token_kind
    }

    fn current_token_is(&self, token_kind: TokenKind) -> bool {
        self.current_token.kind == token_kind
    }

    fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, token_kind: TokenKind) {
        let msg = format!(
            "expected next token to  be {}, got {} instead",
            token_kind, self.peek_token.kind
        );

        self.errors.push(msg)
    }

    fn register_prefix(&mut self, token_kind: TokenKind, prefix_fn: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_kind, prefix_fn);
    }

    fn register_infix(&mut self, token_kind: TokenKind, infix_fn: InfixParseFn) {
        self.infix_parse_fns.insert(token_kind, infix_fn);
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{ExpressionNode, Node, StatementNode};
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
        check_parser_errors(parser);

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

    #[test]
    fn tests_return_statement() {
        let input = r#"
            return 5;
            return 10;
            return 156154;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_errors(parser);

        match program {
            Some(program) => {
                assert_eq!(
                    program.statements.len(),
                    3,
                    "statements does not contain 3 statements. got = {}",
                    program.statements.len()
                );

                for stmt in program.statements {
                    match stmt {
                        StatementNode::Return(ret_stmt) => {
                            assert_eq!(
                                ret_stmt.token_literal(),
                                "return",
                                "token literal not `return`. got={:?}",
                                ret_stmt.token_literal()
                            )
                        }
                        other => panic!("stmt is not ReturnStatement. got={:?}", other),
                    }
                }
            }
            None => panic!("parse program should not be none"),
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        check_parser_errors(parser);

        assert_eq!(
            program.statements.len(),
            1,
            "statements does not contain enough statements, got={}",
            program.statements.len()
        );

        match &program.statements[0] {
            StatementNode::Expression(exp_stmt) => {
                assert!(exp_stmt.expression.is_some());

                match exp_stmt.expression.as_ref().unwrap() {
                    ExpressionNode::IdentifierNode(identifier) => {
                        assert_eq!(
                            identifier.value, "foobar",
                            "identifier value not `foobar`. got={}",
                            identifier.value
                        );
                        assert_eq!(
                            identifier.token_literal(),
                            "foobar",
                            "identifier.token_literal() is not `foobar`. got={}",
                            identifier.token_literal()
                        )
                    }
                    other => panic!("expression not identifier. got={:?}", other),
                }
            }
            other => panic!(
                "program.statements[0] is not ExpressionStatement. got={:?}",
                other
            ),
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        check_parser_errors(parser);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statement does not contain enough statements. got={}",
            program.statements.len()
        );

        match &program.statements[0] {
            StatementNode::Expression(exp_stmt) => {
                assert!(exp_stmt.expression.is_some());
                match exp_stmt.expression.as_ref().unwrap() {
                    ExpressionNode::Integer(integer) => {
                        assert_eq!(
                            integer.value, 5,
                            "integer.value not `5`. got={}",
                            integer.value
                        );
                        assert_eq!(
                            integer.token_literal(),
                            "5",
                            "integer.value not `5`. got={}",
                            integer.token_literal()
                        );
                    }
                    other => panic!("Expression not an IntegerLiteral. got={:?}", other),
                }
            }
            other => panic!(
                "program.statements[0] is not ExpressionStatement. got={:?}",
                other
            ),
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

    fn check_parser_errors(parser: Parser) {
        let errors = parser.errors();

        if errors.len() == 0 {
            return;
        }

        for error in errors {
            eprintln!("parser error: {}", error);
        }

        panic!("parser error present!")
    }
}
