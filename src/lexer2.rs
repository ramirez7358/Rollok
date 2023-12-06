use crate::token::Token;

struct Lexer<'source> {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let lex = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: Default::default(),
        };

        lex.read_position();
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

    fn next_token(&self) -> Token {
        todo()
    }
}

#[cfg(test)]
mod test {
    use crate::lexer2::Lexer;
    use crate::token::{Token, TokenKind};

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

        let lexer = Lexer::new(input);
        for (index, exp_token) in expected.into_iter().enumerate() {
            let receive_token = lexer.next_token();
            assert_eq!(
                exp_token.kind, receive_token,
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
