fn str_is_number(s: &str) -> bool {
    match s.parse::<f64>() {
        Ok(..)  => true,
        Err(..) => false
    }
}

#[derive(Debug, Clone)]
struct Number {
    value: f64
}

#[derive(Debug, Clone)]
struct Vector {
    words: Vec<Word>
}

#[derive(Debug, Clone)]
enum Word {
    Number(Number),
    Vector(Vector),

    Plus,
    Minus,
    Mul,
    Div,

    Print,
    Dup,
}

#[derive(Debug)]
struct Prog {
    stack: Vec<Word>
}

impl Prog {
    fn new() -> Prog {
        Prog { stack: vec!() }
    }

    fn run(&mut self, input: &str) {
        for lexeme in input.split_whitespace() {
            if str_is_number(lexeme) {
                self.stack.push(Word::Number(Number { value: lexeme.parse().unwrap() }));
            } else if lexeme == "+" {
                let a = self.stack.pop().expect("Stack underflow");
                let b = self.stack.pop().expect("Stack underflow");
                if let (Word::Number(A), Word::Number(B)) = (a, b) {
                    self.stack.push(Word::Number(Number { value: A.value+B.value }));
                }
            } else if lexeme == "-" {
                let a = self.stack.pop().expect("Stack underflow");
                let b = self.stack.pop().expect("Stack underflow");
                if let (Word::Number(A), Word::Number(B)) = (a, b) {
                    self.stack.push(Word::Number(Number { value: A.value-B.value }));
                }
            } else if lexeme == "*" {
                let a = self.stack.pop().expect("Stack underflow");
                let b = self.stack.pop().expect("Stack underflow");
                if let (Word::Number(A), Word::Number(B)) = (a, b) {
                    self.stack.push(Word::Number(Number { value: A.value*B.value }));
                }
            } else if lexeme == "/" {
                let a = self.stack.pop().expect("Stack underflow");
                let b = self.stack.pop().expect("Stack underflow");
                if let (Word::Number(A), Word::Number(B)) = (a, b) {
                    self.stack.push(Word::Number(Number { value: A.value/B.value }));
                }
            } else if lexeme == "print" {
                let a = self.stack.pop().expect("Stack underflow");
                println!("{:?}", a);
            } else if lexeme == "dup" {
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(a.clone());
                self.stack.push(a.clone());
            } else {
                panic!("Unknown word");
            }
        }
    }
}

fn main() {
    let input = "1 2 3 dup";
    println!("'{}'", input);
    let mut prog = Prog::new();
    prog.run(input);
    println!("{:?}", prog);
}

