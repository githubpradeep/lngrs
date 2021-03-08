use std::{collections::HashMap};
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,
    STRING,

    ASSIGN,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    EQ,
    NOTEQ,
    BANG,
    LT,
    GT,
    LTE,
    GTE,

    COMA,
    SEMICOLON,
    COLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,


    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN
}

impl From<&str> for TokenType {
    fn from(item: &str) -> Self {
        match item {
            "=" => TokenType::ASSIGN,
            "+" => TokenType::PLUS,
            "," => TokenType::COMA,
            ";" => TokenType::SEMICOLON,
            "(" => TokenType::LPAREN,
            ")" => TokenType::RPAREN,
            "{" => TokenType::LBRACE,
            "}" => TokenType::RBRACE,
            "ILLEGAL" => TokenType::ILLEGAL,
            "EOF" => TokenType::EOF,
            "IDENT" => TokenType::IDENT,
            "INT" => TokenType::INT,
            "FUNCTION" => TokenType::FUNCTION,
            "LET" => TokenType::LET,
            _ => panic!("undeintified token")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String
}

impl Token {
    pub fn new(token_type: TokenType, ch: char) -> Self {
        Token{
            token_type,
            literal: ch.to_string()
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
    keyworks: HashMap<String, TokenType>
}


impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
            keyworks: HashMap::new()
        };
        l.add_keywords();
        l.read_char();
        l
    }

    fn add_keywords(&mut self) {
        self.keyworks.insert("fn".to_string(), TokenType::FUNCTION);
        self.keyworks.insert("let".to_string(), TokenType::LET);
        self.keyworks.insert("true".to_string(), TokenType::TRUE);
        self.keyworks.insert("false".to_string(), TokenType::FALSE);
        self.keyworks.insert("if".to_string(), TokenType::IF);
        self.keyworks.insert("else".to_string(), TokenType::ELSE);
        self.keyworks.insert("return".to_string(), TokenType::RETURN);

    }

    fn lookup_ident(&self, ident: &String) -> TokenType {
        match self.keyworks.get(ident) {
            Some(token_type) => token_type.clone(),
            None => TokenType::IDENT
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while  self.ch == ' ' || self.ch == '\n' || self.ch == '\r' || self.ch == '\t' {
            self.read_char()
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;
        self.skip_whitespace();
        token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token{
                        token_type: TokenType::EQ,
                        literal: "==".to_string()
                    }   
                } else {
                    Token::new(TokenType::ASSIGN, self.ch)
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::NOTEQ,
                        literal: "!=".to_string()
                    }
                } else {
                    Token::new(TokenType::BANG, self.ch)
                }
            },
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::GTE,
                        literal: ">=".to_string()
                    }
                } else {
                    Token::new(TokenType::GT, self.ch)
                }
            },
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::LTE,
                        literal: "<=".to_string()
                    }
                } else {
                    Token::new(TokenType::LT, self.ch)
                }
            },
            ';' => Token::new(TokenType::SEMICOLON, self.ch),
            ':' => Token::new(TokenType::COLON, self.ch),
            '(' => Token::new(TokenType::LPAREN, self.ch),
            ')' => Token::new(TokenType::RPAREN, self.ch),
            '{' => Token::new(TokenType::LBRACE, self.ch),
            '}' => Token::new(TokenType::RBRACE, self.ch),
            '+' => Token::new(TokenType::PLUS, self.ch),
            '-' => Token::new(TokenType::MINUS, self.ch),
            '*' => Token::new(TokenType::ASTERISK, self.ch),
            '/' => Token::new(TokenType::SLASH, self.ch),
            ',' => Token::new(TokenType::COMA, self.ch),
            '[' => Token::new(TokenType::LBRACKET, self.ch),
            ']' => Token::new(TokenType::RBRACKET, self.ch),
            '\0' => Token::new(TokenType::EOF, '\0'),
            '"' => {
                let s = self.read_string();
                Token {
                    token_type: TokenType::STRING,
                    literal: s
                }
            }
            _ => {
                if self.ch.is_ascii_digit() {
                    let number = self.read_number();
                    Token {
                        token_type: TokenType::INT,
                        literal: number 
                    }
                } else if self.ch.is_ascii_alphabetic() {
                    let identifier = self.read_identifier();
                    Token {
                        token_type: self.lookup_ident(&identifier),
                        literal: identifier
                    }
                } else {
                    Token::new(TokenType::ILLEGAL, self.ch)
                }
            }
        };
        self.read_char();
        token

    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while self.peek_char().is_ascii_digit() {
            self.read_char();
        }
        self.input[start..self.read_position].to_string()
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.peek_char().is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        self.input[start..self.read_position].to_string()
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
        }
        self.input[position..self.position].to_string()
    }
}