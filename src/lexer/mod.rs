#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Number,
    LowerLetter,
    UpperLetter,
    GreekLetter,
    DecimalPoint,
    Comma,
    Bang,
    BraceOpen,
    BraceClose,
    CurlyOpen,
    CurlyClose,
    SquareOpen,
    SquareClose,
    Caret,
    VertOpen,
    VertClose,
    VariableSuffix,
    FracStart,
    SumStart,
    ProdStart,
    BinomStart,
    FuncStart,
    SqrtStart,
    Add,
    Sub,
    Mul,
    Div,
    AddSub,
    Equal,
    LessThanEqual,
    GreaterThanEqual,
    LessThan,
    GreaterThan,
    WhiteSpace,
    SemiColon,
    BadSymbol,
    EOF,
}

trait StartCheckable {
    fn starts_lower(&self) -> bool;
    fn starts_upper(&self) -> bool;
    fn starts_digit(&self) -> bool;
    fn starts_greek(&self) -> bool;
    fn starts_whitespace(&self) -> bool;
}

impl StartCheckable for String {
    fn starts_lower(&self) -> bool {
        if let Some(first_char) = self.chars().next() {
            return first_char.is_lowercase();
        }
        false
    }

    fn starts_upper(&self) -> bool {
        if let Some(first_char) = self.chars().next() {
            return first_char.is_uppercase();
        }
        false
    }

    fn starts_digit(&self) -> bool {
        if let Some(first_char) = self.chars().next() {
            return first_char.is_ascii_digit();
        }
        false
    }
    fn starts_whitespace(&self) -> bool {
        if let Some(first_char) = self.chars().next() {
            return first_char.is_whitespace();
        }
        false
    }

    fn starts_greek(&self) -> bool {
        false
    }
}

pub trait Tokenisable {
    fn tokenise(&self) -> Result<Vec<Token>, String>;
}

impl Tokenisable for String {
    fn tokenise(&self) -> Result<Vec<Token>, String> {
        let token_stream = Lexer::new(self.as_str()).tokenise();
        if let Some(bad) = token_stream
            .iter()
            .position(|token| token.token_type == TokenType::BadSymbol)
        {
            println!("{:#?}", token_stream);
            return Err(format!("Token stream contains bad symbol at {}", bad));
        }
        Ok(token_stream)
    }
}

impl Tokenisable for &str {
    fn tokenise(&self) -> Result<Vec<Token>, String> {
        let token_stream = Lexer::new(self).tokenise();
        if let Some(bad) = token_stream
            .iter()
            .position(|token| token.token_type == TokenType::BadSymbol)
        {
            println!("{:#?}", token_stream);
            return Err(format!("Token stream contains bad symbol at {}", bad));
        }
        Ok(token_stream)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct Lexer {
    pos: usize,
    input: String,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            pos: 0,
            input: input.to_string(),
        }
    }

    pub fn tokenise(&mut self) -> Vec<Token> {
        let mut token_stream = Vec::<Token>::new();
        loop {
            let token = self.read_next();
            if token.token_type == TokenType::EOF {
                break;
            }
            token_stream.push(token);
        }
        token_stream.push(Token {
            token_type: TokenType::EOF,
            literal: None,
        });
        token_stream
    }

    fn read_next(&mut self) -> Token {
        if self.pos >= self.input.len() {
            return Token {
                token_type: TokenType::EOF,
                literal: None,
            };
        }

        if let Some(current_input) = self.input.get(self.pos..) {
            let (token_type, len): (TokenType, usize) = match current_input.to_string() {
                t if t.starts_with("\\mathrm") => (TokenType::FuncStart, 7),
                t if t.starts_with("\\lvert") => (TokenType::VertOpen, 6),
                t if t.starts_with("\\prod_") => (TokenType::ProdStart, 6),
                t if t.starts_with("\\sum_") => (TokenType::SumStart, 5),
                t if t.starts_with("\\frac") => (TokenType::FracStart, 5),
                t if t.starts_with("\\sqrt") => (TokenType::SqrtStart, 5),
                t if t.starts_with("\\div") => (TokenType::Div, 4),
                t if t.starts_with("\\pm") => (TokenType::Div, 3),
                t if t.starts_with("<=") => (TokenType::LessThanEqual, 2),
                t if t.starts_with(">=") => (TokenType::GreaterThanEqual, 2),
                t if t.starts_with("=") => (TokenType::Equal, 1),
                t if t.starts_with("(") => (TokenType::BraceOpen, 1),
                t if t.starts_with(")") => (TokenType::BraceClose, 1),
                t if t.starts_with("[") => (TokenType::SquareOpen, 1),
                t if t.starts_with("]") => (TokenType::SquareClose, 1),
                t if t.starts_with("{") => (TokenType::CurlyOpen, 1),
                t if t.starts_with("}") => (TokenType::CurlyClose, 1),
                t if t.starts_with(".") => (TokenType::DecimalPoint, 1),
                t if t.starts_with(",") => (TokenType::Comma, 1),
                t if t.starts_with("!") => (TokenType::Bang, 1),
                t if t.starts_with("+") => (TokenType::Add, 1),
                t if t.starts_with("-") => (TokenType::Sub, 1),
                t if t.starts_with("*") => (TokenType::Mul, 1),
                t if t.starts_with("<") => (TokenType::LessThan, 1),
                t if t.starts_with(">") => (TokenType::GreaterThan, 1),
                t if t.starts_with("^") => (TokenType::Caret, 1),
                t if t.starts_with("/") => (TokenType::Div, 1),
                t if t.starts_with(":") => (TokenType::Div, 1),
                t if t.starts_with(";") => (TokenType::SemiColon, 1),
                t if t.starts_whitespace() => (TokenType::WhiteSpace, 1),
                t if t.starts_lower() => (TokenType::LowerLetter, 1),
                t if t.starts_upper() => (TokenType::UpperLetter, 1),
                t if t.starts_digit() => {
                    let mut len = 0;
                    for i in t.chars() {
                        if (i.is_ascii_digit()) {
                            len += 1;
                        } else {
                            break;
                        }
                    }
                    (TokenType::Number, len)
                }
                _ => (TokenType::BadSymbol, 1),
            };
            self.pos += len;
            if let Some(literal) = current_input.get(..len) {
                return Token {
                    token_type,
                    literal: Some(literal.to_string()),
                };
            } else {
                println!("{}", current_input);
                return Token {
                    token_type: TokenType::EOF,
                    literal: None,
                };
            }
        } else {
            return Token {
                token_type: TokenType::EOF,
                literal: None,
            };
        }
    }
}
