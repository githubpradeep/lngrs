

use iota::iota;
use std::{cell::RefCell, collections::HashMap};

use ast::{Expression, Program, Statement};

use crate::lexer::{Lexer, Token, TokenType};
use crate::ast;

pub type Error = String;

iota! {
    const BLANK: u32 = 1 << iota;
        , LOWEST
        , EQUALS
        , LESSGREATER
        , SUM
        , PRODUCT
        , PREFIX
        , CALL
        , INDEX
}


pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    precedences: HashMap<TokenType, u32>
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::new(TokenType::EOF, '\0'),
            peek_token: Token::new(TokenType::EOF, '\0'),
            precedences: Parser::get_precedences()
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn get_precedences() -> HashMap<TokenType, u32> {
        let mut precedences  = HashMap::new();
        precedences.insert(TokenType::EQ, EQUALS);
        precedences.insert(TokenType::NOTEQ, EQUALS);
        precedences.insert(TokenType::LT, LESSGREATER);
        precedences.insert(TokenType::GT, LESSGREATER);
        precedences.insert(TokenType::LTE, LESSGREATER);
        precedences.insert(TokenType::GTE, LESSGREATER);
        precedences.insert(TokenType::PLUS, SUM);
        precedences.insert(TokenType::MINUS, SUM);
        precedences.insert(TokenType::ASTERISK, PRODUCT);
        precedences.insert(TokenType::SLASH, PRODUCT);
        precedences.insert(TokenType::LPAREN, CALL);
        precedences.insert(TokenType::LBRACKET, INDEX);
        precedences
    }

    fn peek_precedence(&self) -> u32 {
        if let Some(precedence) = self.precedences.get(&self.peek_token.token_type) {
            return precedence.clone();
        }
        LOWEST
    }

    fn cur_precedence(&self) -> u32 {
        if let Some(precedence) = self.precedences.get(&self.cur_token.token_type) {
            return precedence.clone();
        }
        LOWEST
    }

    fn call_prefix(&mut self, token_type: TokenType) -> Result<Expression, Error>{
        match token_type {
            TokenType::IDENT => self.parse_identifier(),
            TokenType::INT => self.parse_integer_literal(),
            TokenType::BANG => self.parse_prefix_expression(),
            TokenType::MINUS => self.parse_prefix_expression(),
            TokenType::TRUE => self.parse_boolean(),
            TokenType::FALSE => self.parse_boolean(),
            TokenType::LPAREN => self.parse_grouped_expression(),
            TokenType::IF => self.parse_if_expression(),
            TokenType::FUNCTION => self.parse_function_literal(),
            TokenType::STRING => self.parse_string_literal(),
            TokenType::LBRACKET => self.parse_array_literal(),
            TokenType::LBRACE => self.parse_hash_literal(),
            _ => Err("prefix function not found".to_string())
        }
    }

    fn call_infix(&mut self, token_type: TokenType, left: Expression) -> Result<Expression, Error>{
        match token_type {
            TokenType::PLUS => self.parse_infix_expression(left),
            TokenType::MINUS => self.parse_infix_expression(left),
            TokenType::SLASH => self.parse_infix_expression(left),
            TokenType::ASTERISK => self.parse_infix_expression(left),
            TokenType::GT => self.parse_infix_expression(left),
            TokenType::LT => self.parse_infix_expression(left),
            TokenType::GTE => self.parse_infix_expression(left),
            TokenType::LTE => self.parse_infix_expression(left),
            TokenType::EQ => self.parse_infix_expression(left),
            TokenType::NOTEQ => self.parse_infix_expression(left),
            TokenType::LPAREN => self.parse_call_expression(left),
            TokenType::LBRACKET => self.parse_index_expression(left),

            _ => Ok(left)
        }
    }
    

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, Error> {
        let mut program = Program::new();
        while self.cur_token.token_type != TokenType::EOF {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token();
        } 
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.cur_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement()
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(LOWEST)?;
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Ok(Statement::ExpressionStatement {
            token,
            expression
        })
    }

    fn parse_block_statement(&mut self) -> Result<Statement, Error> {
        let token = self.cur_token.clone();
        let mut statements = Vec::new();
        self.next_token();
        while !self.cur_token_is(TokenType::RBRACE) && !self.cur_token_is(TokenType::EOF) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }
        Ok(Statement::BlockStatment {
            token,
            statements
        })
    }

    fn parse_expression(&mut self, precedence: u32) -> Result<Expression, Error> {
        let left_exp = self.call_prefix(self.cur_token.token_type)?;
        let left_rc = RefCell::new(left_exp);
        while !self.peek_token_is(TokenType::SEMICOLON) && precedence < self.peek_precedence() {
            let infix_token = self.peek_token.token_type.clone();
            self.next_token();
            let left = left_rc.replace(Expression::Nope);
            if let Ok(exp) = self.call_infix(infix_token, left) {
                left_rc.replace(exp);
            }
        }
        return Ok(left_rc.into_inner());
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let expression = self.parse_expression(PREFIX)?;
        Ok(Expression::PrefixExpression {
            token,
            operator,
            right: Box::new(expression)
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Ok(Expression::InfixExpression {
            token,
            operator,
            left: Box::new(left),
            right: Box::new(right)
        })
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        let arguments = self.parse_call_arguments()?;
        Ok(Expression::CallExpression {
            token,
            function: Box::new(function),
            arguments
        })
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, Error> {
        let mut args = Vec::new();
        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return Ok(args);
        }
        self.next_token();
        let arg = self.parse_expression(LOWEST)?;
        args.push(arg);
        while self.peek_token_is(TokenType::COMA) {
            self.next_token();
            self.next_token();
            let arg = self.parse_expression(LOWEST)?;
            args.push(arg);
        }
        if !self.expect_peek(TokenType::RPAREN) {
            return Err("expceted ) after arguments".to_string());
        }
        Ok(args)
    }

    fn parse_index_expression(&mut self, left: Expression) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        self.next_token();
        let index_exp = self.parse_expression(LOWEST)?;
        if !self.expect_peek(TokenType::RBRACKET) {
            return Err("array index should end with ]".to_string());
        }
        Ok(Expression::IndexExpression {
            token,
            left: Box::new(left),
            index: Box::new(index_exp)
       })
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, Error> {
        self.next_token();
        let exp = self.parse_expression(LOWEST)?;
        if !self.expect_peek(TokenType::RPAREN) {
            return Err("missing )".to_string());
        }
        Ok(exp)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::LPAREN) {
            return Err("expected ( after if".to_string());
        }
        self.next_token();
        let condition = self.parse_expression(LOWEST)?;
        if !self.expect_peek(TokenType::RPAREN) {
            return Err("expected ) after condition".to_string());
        }
        if !self.expect_peek(TokenType::LBRACE) {
            return Err("expected { after if condition".to_string());
        }
        let consequence = self.parse_block_statement()?;

        if self.peek_token_is(TokenType::ELSE) {
            self.next_token();
            if !self.expect_peek(TokenType::LBRACE) {
                return Err("expected { after else condition".to_string());
            }
            let alternative = self.parse_block_statement()?;
            return Ok(Expression::IfExpression {
                token,
                condition: Box::new(condition),
                consequence: Box::new(consequence),
                alternative: Some(Box::new(alternative))
            });

        }
        Ok(Expression::IfExpression {
            token,
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: None
        })

    }

    fn parse_function_literal(&mut self) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::LPAREN) {
            return Err("expected ( after function".to_string());
        }
        let parameters = self.parse_function_parameters()?;
        if !self.expect_peek(TokenType::LBRACE) {
            return Err("expected { before function body".to_string());
        }
        let body = self.parse_block_statement()?;
        Ok(Expression::FunctionLiteral{
            token,
            parameters,
            body: Box::new(body)
        })
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Expression>, Error> {
        let mut identifiers = Vec::new();
        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return Ok(identifiers);
        }
        self.next_token();
        let identifier = Expression::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone()
        };
        identifiers.push(identifier);

        while self.peek_token_is(TokenType::COMA) {
            self.next_token();
            self.next_token();
            let identifier = Expression::Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone()
            };
            identifiers.push(identifier);
        }
        if !self.expect_peek(TokenType::RPAREN) {
            return Err("expected ) after function parameters".to_string());
        }
        Ok(identifiers)
    }

    fn parse_identifier(&mut self) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        Ok(Expression::Identifier {
            token,
            value: self.cur_token.literal.clone()
        })
    }

    fn parse_integer_literal(&mut self) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        let number  = self.cur_token.literal.parse();
        if number.is_err() {
            return Err("cannot parse string as number".to_string());
        }
        Ok(Expression::IntegerLiteral{
            token,
            value: number.unwrap()
        })
    }

    fn parse_array_literal(&mut self) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        let elements = self.parse_expression_list(TokenType::RBRACKET)?;
        Ok(Expression::ArrayLiteral {
            token,
            elements
        })
    }

    fn parse_hash_literal(&mut self) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        let mut pairs = Vec::new();
        while !self.peek_token_is(TokenType::RBRACE) {
            self.next_token();
            let key = self.parse_expression(LOWEST)?;
            if !self.expect_peek(TokenType::COLON) {
                return Err("incorrect hashmap".to_string());
            }
            self.next_token();
            let value = self.parse_expression(LOWEST)?;
            pairs.push((key, value));
            if !self.peek_token_is(TokenType::RBRACE) && !self.peek_token_is(TokenType::COMA) {
                return Err("missing , hashmap".to_string());
            }
            if self.peek_token_is(TokenType::COMA) {
                self.next_token();
            }
        }
        if !self.expect_peek(TokenType::RBRACE) {
            return Err("unednig hashmap".to_string());
        }
        Ok(Expression::HashLiteral {
            pairs,
            token
        })
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Result<Vec<Expression>, Error> {
        let mut result = Vec::new();
        if self.peek_token_is(end) {
            self.next_token();
            return Ok(result);
        }
        self.next_token();
        let expression = self.parse_expression(LOWEST)?;
        result.push(expression);
        while self.peek_token_is(TokenType::COMA) {
            self.next_token();
            self.next_token();
            let expression = self.parse_expression(LOWEST)?;
            result.push(expression);
        }
        if !self.expect_peek(end) {
            return Err("array must end with ]".to_string());
        }
        Ok(result)
    }

    fn parse_string_literal(&mut self) -> Result<Expression, Error> {
        let token = self.cur_token.clone();
        Ok(Expression::StringLiteral{
            token,
            value: self.cur_token.literal.clone()
        })
    }

    fn parse_boolean(&mut self) -> Result<Expression, Error> { 
        Ok(Expression::Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::TRUE)
        })
    }


    fn parse_let_statement(&mut self) -> Result<Statement, Error> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::IDENT) {
            Err("identifier expected".to_string())
        } else {
            let name = Expression::Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.clone().literal
            };
            if !self.expect_peek(TokenType::ASSIGN) {
                return Err("identifier expected".to_string());
            }
            self.next_token();
            
            let expression = self.parse_expression(LOWEST)?;
            
            if self.peek_token_is(TokenType::SEMICOLON) {
                self.next_token();
            }
            Ok(Statement::LetStatement {
                token,
                name,
                value: expression
            })
        }
    }

    fn parse_return_statement(&mut self) -> Result<Statement, Error> {
        let token = self.cur_token.clone();
        self.next_token();
        let return_value = self.parse_expression(LOWEST)?;
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Ok(Statement::ReturnStatement {
            token,
            return_value
        })
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool  {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        }
        return false;
    }
}
