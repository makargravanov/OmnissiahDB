use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
enum SQLExpression {
    Identifier(String),
    Literal(String),
    Operator(String),
    BinaryExpression {
        operator: String,
        left: Box<SQLExpression>,
        right: Box<SQLExpression>,
    },
}

#[derive(Debug)]
struct SelectQuery {
    columns: Vec<SQLExpression>,
    table: SQLExpression,
    conditions: Option<SQLExpression>,
}

#[derive(Debug)]
struct InsertQuery {
    table: SQLExpression,
    columns: Vec<SQLExpression>,
    values: Vec<SQLExpression>,
}

#[derive(Debug)]
pub enum SQLQuery {
    Select(SelectQuery),
    Insert(InsertQuery),
}

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, String>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<String>) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Option<SQLQuery> {
        match self.tokens.peek()?.as_str() {
            "SELECT" => self.parse_select().map(SQLQuery::Select),
            "INSERT" => self.parse_insert().map(SQLQuery::Insert),
            _ => None,
        }
    }

    fn parse_select(&mut self) -> Option<SelectQuery> {
        self.expect_keyword("SELECT")?;
        let columns = self.parse_columns()?;
        self.expect_keyword("FROM")?;
        let table = self.parse_identifier()?;
        let conditions = if self.peek_keyword("WHERE") {
            self.next_token();
            self.parse_conditions()
        } else {
            None
        };
        Some(SelectQuery {
            columns,
            table,
            conditions,
        })
    }

    fn parse_insert(&mut self) -> Option<InsertQuery> {
        self.expect_keyword("INSERT")?;
        self.expect_keyword("INTO")?;
        let table = self.parse_identifier()?;
        self.expect_token("(")?;
        let columns = self.parse_columns()?;
        self.expect_token(")")?;
        self.expect_keyword("VALUES")?;
        self.expect_token("(")?;
        let values = self.parse_values()?;
        self.expect_token(")")?;
        Some(InsertQuery {
            table,
            columns,
            values,
        })
    }
    fn parse_columns(&mut self) -> Option<Vec<SQLExpression>> {
        let mut columns = Vec::new();
        loop {
            if let Some(column) = self.parse_identifier() {
                columns.push(column);
            } else {
                return None;
            }
            if !self.peek_token(",") {
                break;
            }
            self.next_token();
        }
        Some(columns)
    }

    fn parse_values(&mut self) -> Option<Vec<SQLExpression>> {
        let mut values = Vec::new();
        loop {
            if let Some(value) = self.parse_literal() {
                values.push(value);
            } else {
                return None;
            }
            if !self.peek_token(",") {
                break;
            }
            self.next_token();
        }
        Some(values)
    }

    fn parse_conditions(&mut self) -> Option<SQLExpression> {
        self.parse_logical_expression()
    }

    fn parse_logical_expression(&mut self) -> Option<SQLExpression> {
        let mut left = self.parse_comparison_expression()?;

        while let Some(operator) = self.peek_operator() {
            if operator == "AND" || operator == "OR" {
                self.next_token();
                let right = self.parse_comparison_expression()?;
                left = SQLExpression::BinaryExpression {
                    operator,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Some(left)
    }

    fn parse_comparison_expression(&mut self) -> Option<SQLExpression> {
        let mut left = self.parse_primary_expression()?;

        while let Some(operator) = self.peek_operator() {
            if operator == "==" || operator == "!=" || operator == "<" || operator == ">" || operator == "<=" || operator == ">=" {
                self.next_token();
                let right = self.parse_primary_expression()?;
                left = SQLExpression::BinaryExpression {
                    operator,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Some(left)
    }

    fn parse_primary_expression(&mut self) -> Option<SQLExpression> {
        if self.peek_token("(") {
            self.next_token();
            let expr = self.parse_logical_expression();
            self.expect_token(")")?;
            expr
        } else if let Some(identifier) = self.parse_identifier() {
            Some(identifier)
        } else {
            self.parse_literal()
        }
    }

    fn peek_operator(&mut self) -> Option<String> {
        if let Some(token) = self.tokens.peek() {
            let token_str = token.as_str();
            if token_str == "AND" || token_str == "OR" || token_str == "==" || token_str == "!=" || token_str == "<" || token_str == ">" || token_str == "<=" || token_str == ">=" {
                return Some(token_str.to_string());
            }
        }
        None
    }

    fn parse_identifier(&mut self) -> Option<SQLExpression> {
        if let Some(token) = self.next_token() {
            Some(SQLExpression::Identifier(token.clone()))
        } else {
            None
        }
    }

    fn parse_literal(&mut self) -> Option<SQLExpression> {
        if let Some(token) = self.next_token() {
            Some(SQLExpression::Literal(token.clone()))
        } else {
            None
        }
    }

    fn expect_keyword(&mut self, keyword: &str) -> Option<()> {
        if self.next_token() == Some(&keyword.to_string()) {
            Some(())
        } else {
            None
        }
    }

    fn expect_token(&mut self, token: &str) -> Option<()> {
        if self.next_token() == Some(&token.to_string()) {
            Some(())
        } else {
            None
        }
    }

    fn peek_keyword(&mut self, keyword: &str) -> bool {
        self.tokens.peek() == Some(&&keyword.to_string())
    }

    fn peek_token(&mut self, token: &str) -> bool {
        self.tokens.peek() == Some(&&token.to_string())
    }

    fn next_token(&mut self) -> Option<&String> {
        self.tokens.next()
    }
}