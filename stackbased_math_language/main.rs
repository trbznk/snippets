fn string_is_number(s: String) -> bool {
    match s.parse::<f64>() {
        Ok(..)  => true,
        Err(..) => false
    }
}

#[derive(Debug)]
enum TokenType {
    NUMBER,
    PLUS,
    UNDEFINED
}

#[derive(Debug)]
struct Token {
    lexeme: String,
    tp: TokenType
}

impl Token {
    fn new() -> Token {
        Token {
            lexeme: String::new(),
            tp: TokenType::UNDEFINED 
        }
    }
}

#[derive(Debug)]
struct Lexer {
    tokens: Vec<Token>
}

impl Lexer {
    fn new() -> Lexer {
        Lexer { tokens: Vec::new() }
    }

    fn from(input: String) -> Lexer {
        let mut lexer = Lexer::new(); 
        for lexeme in input.split_whitespace() {
            let mut token = Token::new();
            if lexeme == "+" {
                token.tp = TokenType::PLUS;
            } else if string_is_number(lexeme.to_string()) {
                token.tp = TokenType::NUMBER;
            }
            token.lexeme = lexeme.to_string();
            lexer.tokens.push(token);
        }

        lexer
    }

    fn print(&self) {
        for token in &self.tokens {
            println!("{:?}", token);
        }
    }
}

fn main() {
    let input = String::from("1 2 +");
    println!("input: '{}'", input);
    let lexer = Lexer::from(input);
    lexer.print();
}
