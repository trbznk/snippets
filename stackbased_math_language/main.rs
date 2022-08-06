use std::collections::HashMap;

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
    stack: Vec<Word>,
    reg: HashMap<String, Word>
}

impl Prog {
    fn new() -> Prog {
        Prog {
            stack: vec!(),
            reg: HashMap::new()
        }
    }
    
    fn preprocess(&mut self, input: &str) {




        // todo!();
    }

    fn run(&mut self, input: &str) {
        self.preprocess(input);
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
            } else if lexeme.starts_with("@") {
                if lexeme.len() == 1 {
                    panic!("Syntax Error");
                }
                let a = self.stack.pop().expect("Stack underflow");
                let var_name = lexeme.strip_prefix("@").unwrap();
                self.reg.insert(var_name.to_string(), a);
            } else if self.reg.contains_key(lexeme) {
                let a = self.reg.get(lexeme).unwrap();
                self.stack.push((*a).clone());
            } else {
                panic!("Unknown word");
            }
        }
    }
}

fn main() {
    let input = "3 @a 2 a +";
    println!("'{}'", input);
    let mut prog = Prog::new();
    prog.run(input);
    println!("{:?}", prog);
}

