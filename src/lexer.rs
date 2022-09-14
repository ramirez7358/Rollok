use std::str::CharIndices;

static KEYWORDS: [&str;2] = ["var", "object"];

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenValue<'source> {
    EOF,
    Error,
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    LeftBrace,
    RightBrace,
    Number(&'source str),
    Keyword(&'source str),
    Identifier(&'source str),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token<'source> {
    pub value: TokenValue<'source>,
    pub pos: usize,
}

pub struct Lexer<'source> {
    input: &'source str,
    iter: CharIndices<'source>,
    c: char,
    ci: usize,
    error: bool
}

impl <'source> Lexer<'source> {
    pub fn new(input: &'source str) -> Self {
        let mut lex = Self {
            input,
            iter: input.char_indices(),
            c: '\x00',
            ci: 0,
            error: false
        };

        lex.scan_char();
        lex
    }

    pub fn next_token(&mut self) -> Token<'source> {
        self.skip_nontokens();

        if self.is_at_end() {
            return Token {
                value: TokenValue::EOF,
                pos: self.ci
            }
        }

        let token_value = match self.c {
            '+' => TokenValue::Plus,
            '-' => TokenValue::Minus,
            '*' => TokenValue::Multiply,
            '/' => TokenValue::Divide,
            '=' => TokenValue::Equals,
            '{' => TokenValue::LeftBrace,
            '}' => TokenValue::RightBrace,
            _   => TokenValue::Error
        };

        if token_value != TokenValue::Error {
            // If an operator matched, return it as a token
            let token = Token {
                value: token_value,
                pos: self.ci,
            };
            self.scan_char();
            token
        } else if self.c.is_alphabetic() || self.c == '_' || self.c == '$' {
            self.scan_identifier()
        } else if self.c.is_digit(10) {
            self.scan_number()
        } else {
            self.error_token()
        }
    }

    fn scan_identifier(&mut self) -> Token<'source> {
        let startpos = self.ci;
        while self.c.is_alphanumeric() || self.c == '_' || self.c == '$' {
            self.scan_char();
        }
        let token_text = &self.input[startpos..self.ci];
        Token {
            value: if Self::is_keyword(token_text) {TokenValue::Keyword(token_text)} else {TokenValue::Identifier(token_text)},
            pos: startpos,
        }
    }

    fn is_keyword(word: &str) -> bool {
        KEYWORDS.contains(&word)
    }

    fn scan_number(&mut self) -> Token<'source> {
        let startpos = self.ci;
        while self.c.is_digit(10) {
            self.scan_char();
        }
        Token {
            value: TokenValue::Number(&self.input[startpos..self.ci]),
            pos: startpos,
        }
    }

    fn error_token(&mut self) -> Token<'source> {
        self.error = true;
        Token {
            value: TokenValue::Error,
            pos: self.ci,
        }
    }

    fn is_at_end(&self) -> bool {
        self.ci >= self.input.len()
    }

    fn skip_nontokens(&mut self) {
        while self.c == ' ' || self.c == '\t' || self.c == '\r' || self.c == '\n' {
            self.scan_char();
        } 
    }

    fn scan_char(&mut self) {
        if let Some((index, chr)) = self.iter.next() {
            self.ci = index;
            self.c = chr;
        } else {
            self.ci = self.input.len();
            self.c = '\x00';
        }
    }
}

// Lexer is an Iterator; it returns tokens until EOF is encountered, when it
// returns None (the EOF token itself is not returned). Note that errors are
// still returned as tokens with TokenValue::Error.
impl<'source> Iterator for Lexer<'source> {
    type Item = Token<'source>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            // If an error has already been set before we invoke next_token,
            // it means we've already returned TokenValue::Error once and now
            // we should terminate the iteration.
            return None;
        }

        let tok = self.next_token();
        if tok.value == TokenValue::EOF {
            None
        } else {
            Some(tok)
        }
    }
}

pub fn tokenize_all_collect<'source>(data: &'source str) -> Vec<Token<'source>> {
    let lex = Lexer::new(data);
    lex.collect()
}