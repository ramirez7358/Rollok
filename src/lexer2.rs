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
        self.skip_whitespaces();
        let token = match self.ch {
            '+' | '-' | '*' | '/' | '=' | ':' | ';' | ',' | '(' | ')' | '[' | ']' | '{' | '}'
            | '!' | '#' | '>' | '<' => {
                let t = Lexer::new_token(Lexer::match_token_kind(self.ch), self.ch);
                self.read_char();
                t
            }
            '\0' => Token {
                kind: TokenKind::EOF,
                literal: "".to_string(),
            },
            ch if Lexer::is_letter(ch) => {
                let literal = self.read_identifier();
                let kind = lookup_ident(&literal);
                Token { kind, literal }
            }
            ch if Lexer::is_digit(ch) => {
                let kind = TokenKind::Number;
                let literal = self.read_number();
                Token { kind, literal }
            }
            _ => Lexer::new_token(TokenKind::Error, self.ch),
        };

        token
    }

    fn match_token_kind(ch: char) -> TokenKind {
        match ch {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Divide,
            '=' => TokenKind::Assign,
            ':' => TokenKind::Colon,
            ';' => TokenKind::SemiColon,
            ',' => TokenKind::Comma,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '!' => TokenKind::Bang,
            '#' => TokenKind::Slash,
            '>' => TokenKind::GreaterThan,
            '<' => TokenKind::LessThan,
            _ => TokenKind::Error,
        }
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn skip_whitespaces(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char()
        }
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

    fn is_digit(ch: char) -> bool {
        ch.is_numeric()
    }

    fn read_number(&mut self) -> String {
        let mut num = String::from("");
        while Lexer::is_digit(self.ch) {
            num.push(self.ch);
            self.read_char();
        }
        num
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

            !-/*5;
            5 < 10 > 5;
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
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Identifier,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::LeftParen,
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
                kind: TokenKind::RightParen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::SemiColon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Bang,
                literal: "!".to_string(),
            },
            Token {
                kind: TokenKind::Minus,
                literal: "-".to_string(),
            },
            Token {
                kind: TokenKind::Divide,
                literal: "/".to_string(),
            },
            Token {
                kind: TokenKind::Multiply,
                literal: "*".to_string(),
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
                kind: TokenKind::Number,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::LessThan,
                literal: "<".to_string(),
            },
            Token {
                kind: TokenKind::Number,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::GreaterThan,
                literal: ">".to_string(),
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
                kind: TokenKind::LeftParen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::RightParen,
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
