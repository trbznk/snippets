fn string_is_number(s: &str) -> bool {
    match s.parse::<f64>() {
        Ok(..)  => true,
        Err(..) => false
    }
}

#[derive(Debug, PartialEq)]
enum WordType {
    NUMBER,
    PLUS,
    MINUS,
    MUL,
    DIV,
    UNDEFINED
}

#[derive(Debug)]
struct Word {
    data: String,
    tp: WordType,
}

impl Word {
    fn new(data: String, tp: WordType) -> Word {
        Word {
            data: data,
            tp: tp
        }
    }

    fn plus(&self, word: Word) -> Word {
        if self.tp == WordType::NUMBER && word.tp == WordType::NUMBER {
            let r = self.data.parse::<f64>().unwrap() + word.data.parse::<f64>().unwrap();
            return Word::new(r.to_string(), WordType::NUMBER);
        }
        Word::new("".to_string(), WordType::UNDEFINED)
    }

    fn minus(&self, word: Word) -> Word {
        if self.tp == WordType::NUMBER && word.tp == WordType::NUMBER {
            let r = self.data.parse::<f64>().unwrap() - word.data.parse::<f64>().unwrap();
            return Word::new(r.to_string(), WordType::NUMBER);
        }
        Word::new("".to_string(), WordType::UNDEFINED)
    }
    
    fn mul(&self, word: Word) -> Word {
        if self.tp == WordType::NUMBER && word.tp == WordType::NUMBER {
            let r = self.data.parse::<f64>().unwrap() * word.data.parse::<f64>().unwrap();
            return Word::new(r.to_string(), WordType::NUMBER);
        }
        Word::new("".to_string(), WordType::UNDEFINED)
    }

    fn div(&self, word: Word) -> Word {
        if self.tp == WordType::NUMBER && word.tp == WordType::NUMBER {
            let r = self.data.parse::<f64>().unwrap() / word.data.parse::<f64>().unwrap();
            return Word::new(r.to_string(), WordType::NUMBER);
        }
        Word::new("".to_string(), WordType::UNDEFINED)
    }
}

#[derive(Debug)]
struct Prog {
    stack: Vec<Word>
}

impl Prog {
    fn new() -> Prog {
        Prog { stack: vec!() }
    }

    fn push(&mut self, word: Word) {
        self.stack.push(word);
    }

    fn pop(&mut self) -> Word {
        self.stack.pop().expect("ERROR: Stack underflow")
    }

    fn run(&mut self, word: Word) {
        match word.tp {
            WordType::NUMBER => {
                self.push(word);
            },
            WordType::PLUS => {
                let b = self.pop();
                let a = self.pop();
                let c = a.plus(b);
                self.push(c);
            },
            WordType::MINUS => {
                let b = self.pop();
                let a = self.pop();
                let c = a.minus(b);
                self.push(c);
            },
            WordType::MUL => {
                let b = self.pop();
                let a = self.pop();
                let c = a.mul(b);
                self.push(c);
            },
            WordType::DIV => {
                let b = self.pop();
                let a = self.pop();
                let c = a.div(b);
                self.push(c);
            },
            WordType::UNDEFINED => {
                panic!("Unknown word");
            },
        }
    }

    fn run_string(&mut self, s: &str) {
        if s == "+" {
            self.run(Word::new(s.to_string(), WordType::PLUS));
        } else if string_is_number(s) {
            self.run(Word::new(s.to_string(), WordType::NUMBER));
        } else if s == "-" {
            self.run(Word::new(s.to_string(), WordType::MINUS));
        } else if s == "*" {
            self.run(Word::new(s.to_string(), WordType::MUL));
        } else if s == "/" {
            self.run(Word::new(s.to_string(), WordType::DIV));
        } else {
            self.run(Word::new(s.to_string(), WordType::UNDEFINED));
        }
    }
}

fn main() {
    let input = String::from("2 3 /");
    println!("input: '{}'", input);
    let mut prog = Prog::new();
    for s in input.split_whitespace() {
        println!("{:?}", prog);
        prog.run_string(s);
    }
    println!("{:?}", prog);
}

