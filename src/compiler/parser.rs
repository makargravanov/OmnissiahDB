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
    CreateColumn {
        name: String,
        data_type: DataType,
        is_primary: bool,
        not_null: bool,
        identity: bool,
    },
}

#[derive(Debug)]
enum DataType {
    TEXT { length: i32 },
    BOOL,
    DATETIME,

    FLOAT { unsigned: bool },
    DOUBLE { unsigned: bool },

    BYTE { unsigned: bool },
    SHORT { unsigned: bool },
    INT { unsigned: bool },
    LONG { unsigned: bool },
    BIGINT { unsigned: bool },
    UUID { unsigned: bool },
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
struct CreateQuery {
    created: String,
    name: String,
    columns: Option<Vec<SQLExpression>>,
}

#[derive(Debug)]
pub enum SQLQuery {
    Select(SelectQuery),
    Insert(InsertQuery),
    Create(CreateQuery),
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
            "CREATE" => self.parse_create().map(SQLQuery::Create),
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
    fn parse_create(&mut self) -> Option<CreateQuery> {
        self.expect_keyword("CREATE")?;
        if let Some(token) = self.tokens.next() {
            let token_str = token.as_str();
            let created;
            let name;
            let columns;
            if token_str == "TABLE" {
                created = token_str.to_string();
                name = self.tokens.next()?.to_string();
                let mut cols = Vec::new();
                self.expect_token("(")?;
                loop {
                    let name;
                    let data_type;
                    let mut data_type_case;
                    let mut is_primary = false;
                    let mut not_null = false;
                    let mut identity = false;
                    if let Some(column) = self.tokens.next() {
                        name = column.to_string();

                        if let Some(column) = self.tokens.next() {
                            data_type = column.to_string().to_uppercase();

                            match data_type.as_str() {
                                "TEXT" => data_type_case = DataType::TEXT { length: 256 },
                                "BOOL" => data_type_case = DataType::BOOL,
                                "DATETIME" => data_type_case = DataType::DATETIME,

                                "FLOAT" => data_type_case = DataType::FLOAT { unsigned: false },
                                "DOUBLE" => data_type_case = DataType::DOUBLE { unsigned: false },
                                "BYTE" => data_type_case = DataType::BYTE { unsigned: false },
                                "SHORT" => data_type_case = DataType::SHORT { unsigned: false },
                                "INT" => data_type_case = DataType::INT { unsigned: false },
                                "LONG" => data_type_case = DataType::LONG { unsigned: false },
                                "BIGINT" => data_type_case = DataType::BIGINT { unsigned: false },
                                "UUID" => data_type_case = DataType::UUID { unsigned: false },

                                "U_FLOAT" => data_type_case = DataType::FLOAT { unsigned: true },
                                "U_DOUBLE" => data_type_case = DataType::DOUBLE { unsigned: true },
                                "U_BYTE" => data_type_case = DataType::BYTE { unsigned: true },
                                "U_SHORT" => data_type_case = DataType::SHORT { unsigned: true },
                                "U_INT" => data_type_case = DataType::INT { unsigned: true },
                                "U_LONG" => data_type_case = DataType::LONG { unsigned: true },
                                "U_BIGINT" => data_type_case = DataType::BIGINT { unsigned: true },
                                "U_UUID" => data_type_case = DataType::UUID { unsigned: true },
                                _ => return None,
                            }

                            if let Some(column) = self.tokens.next() {
                                if (column.to_string() == "PRIMARY") {
                                    is_primary = true;
                                } else if (column.to_string() == "NOTNULL") {
                                    not_null = true;
                                } else if (column.to_string() == "IDENTITY") {
                                    identity = true;
                                } else if (column.to_string() == ",") {
                                    cols.push(SQLExpression::CreateColumn {
                                        name,
                                        data_type: data_type_case,
                                        is_primary,
                                        not_null,
                                        identity,
                                    });
                                    continue;
                                } else if (column.to_string() == ")") {
                                    cols.push(SQLExpression::CreateColumn {
                                        name,
                                        data_type: data_type_case,
                                        is_primary,
                                        not_null,
                                        identity,
                                    });
                                    break;
                                } else {
                                    return None;
                                }
                                if let Some(column) = self.tokens.next() {
                                    if (column.to_string() == "NOTNULL") {
                                        not_null = true;
                                    } else if (column.to_string() == "IDENTITY") {
                                        identity = true;
                                    } else if (column.to_string() == ",") {
                                        cols.push(SQLExpression::CreateColumn {
                                            name,
                                            data_type: data_type_case,
                                            is_primary,
                                            not_null,
                                            identity,
                                        });
                                        continue;
                                    } else if (column.to_string() == ")") {
                                        cols.push(SQLExpression::CreateColumn {
                                            name,
                                            data_type: data_type_case,
                                            is_primary,
                                            not_null,
                                            identity,
                                        });
                                        break;
                                    } else {
                                        return None;
                                    }
                                    if let Some(column) = self.tokens.next() {
                                        if (column.to_string() == "IDENTITY") {
                                            identity = true;
                                        } else if (column.to_string() == ",") {
                                            cols.push(SQLExpression::CreateColumn {
                                                name,
                                                data_type: data_type_case,
                                                is_primary,
                                                not_null,
                                                identity,
                                            });
                                            continue;
                                        } else if (column.to_string() == ")") {
                                            cols.push(SQLExpression::CreateColumn {
                                                name,
                                                data_type: data_type_case,
                                                is_primary,
                                                not_null,
                                                identity,
                                            });
                                            break;
                                        } else {
                                            return None;
                                        }
                                        if let Some(column) = self.tokens.next() {
                                            if (column.to_string() == ",") {
                                                cols.push(SQLExpression::CreateColumn {
                                                    name,
                                                    data_type: data_type_case,
                                                    is_primary,
                                                    not_null,
                                                    identity,
                                                });
                                                continue;
                                            } else if (column.to_string() == ")") {
                                                cols.push(SQLExpression::CreateColumn {
                                                    name,
                                                    data_type: data_type_case,
                                                    is_primary,
                                                    not_null,
                                                    identity,
                                                });
                                                break;
                                            } else {
                                                return None;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                self.expect_token(";")?;

                columns = Some(cols);
            } else if token_str == "DATABASE" {
                created = token_str.to_string();
                name = self.tokens.next()?.to_string();
                if self.tokens.next()?.to_string() != ";" {
                    return None;
                };
                columns = None;
            } else {
                return None;
            }
            Some(CreateQuery {
                created,
                name,
                columns,
            })
        } else {
            None
        }
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
            if operator == "=="
                || operator == "!="
                || operator == "<"
                || operator == ">"
                || operator == "<="
                || operator == ">="
            {
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
            if token_str == "AND"
                || token_str == "OR"
                || token_str == "=="
                || token_str == "!="
                || token_str == "<"
                || token_str == ">"
                || token_str == "<="
                || token_str == ">="
            {
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
