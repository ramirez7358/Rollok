use crate::token::{lookup_ident, Token, TokenKind};

struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lex = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: Default::default(),
        };

        lex.read_char();
        lex
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '+' => Lexer::new_token(TokenKind::Plus, self.ch),
            '-' => Lexer::new_token(TokenKind::Minus, self.ch),
            '*' => Lexer::new_token(TokenKind::Multiply, self.ch),
            '/' => Lexer::new_token(TokenKind::Divide, self.ch),
            '=' => Lexer::new_token(TokenKind::Assign, self.ch),
            ':' => Lexer::new_token(TokenKind::Colon, self.ch),
            ';' => Lexer::new_token(TokenKind::SemiColon, self.ch),
            ',' => Lexer::new_token(TokenKind::Comma, self.ch),
            '(' => Lexer::new_token(TokenKind::LeftParent, self.ch),
            ')' => Lexer::new_token(TokenKind::RightParent, self.ch),
            '[' => Lexer::new_token(TokenKind::LeftAng, self.ch),
            ']' => Lexer::new_token(TokenKind::RightAng, self.ch),
            '{' => Lexer::new_token(TokenKind::LeftBrace, self.ch),
            '}' => Lexer::new_token(TokenKind::RightBrace, self.ch),
            '\0' => Token {
                kind: TokenKind::EOF,
                literal: "".to_string(),
            },
            _ => {
                if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = lookup_ident(&literal);
                    Token { kind, literal }
                } else {
                    Lexer::new_token(TokenKind::Error, self.ch)
                }
            }
        };

        self.read_char();

        return token;
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn new_token(kind: TokenKind, ch: char) -> Token {
        Token {
            kind,
            literal: ch.to_string(),
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while Lexer::is_letter(self.ch) {
            identifier.push(self.ch);
            self.read_char();
        }
        identifier
    }
}

#[cfg(test)]
mod test {
    use crate::lexer2::Lexer;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_program() {
        let input = r#"
            var five = 5;
            const six = 6;

            var add = {x, y => x + y};
            var result = add(five, six);
        "#;

        let expected: Vec<Token> = vec![
            Token {
                kind: TokenKind::Var,
                literal: "var".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Number,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::SemiColon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Const,
                literal: "const".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "six".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Number,
                literal: "6".to_string(),
            },
            Token {
                kind: TokenKind::SemiColon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Var,
                literal: "var".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::GreaterThan,
                literal: ">".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::SemiColon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Var,
                literal: "var".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "result".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::LeftParent,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "six".to_string(),
            },
            Token {
                kind: TokenKind::RightParent,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::SemiColon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::EOF,
                literal: "".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);
        exec_assert(expected, &mut lexer);
    }

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let expected: Vec<Token> = vec![
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::LeftParent,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::RightParent,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::RightBrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::SemiColon,
                literal: ";".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);
        exec_assert(expected, &mut lexer);
    }

    fn exec_assert(expected: Vec<Token>, lexer: &mut Lexer) {
        for (index, exp_token) in expected.into_iter().enumerate() {
            let receive_token = lexer.next_token();
            assert_eq!(
                exp_token.kind, receive_token.kind,
                "tests[{index}] - token type wrong. Expected={:?}, got={:?}",
                exp_token.kind, receive_token.kind
            );

            assert_eq!(
                exp_token.literal, receive_token.literal,
                "tests[{index}] - literal wrong. Expected={}, got={}",
                exp_token.literal, receive_token.literal
            );
        }
    }
}
