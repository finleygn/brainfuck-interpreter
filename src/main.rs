use std::io::Read;

pub struct Program {
    mem: [u8; 30000],
    ptr: usize,
    lst: Vec<usize>,
    idx: usize,
}

impl Program {
    pub fn new() -> Program {
        Program {
            mem: [0u8; 30000], // Memory allocation
            ptr: 0,            // Current memory address
            idx: 0,            // Current index to be interpreted
            lst: vec![],       // Loop stack start indexes
        }
    }

    pub fn run(&mut self, input: &str) {
        let operators: Vec<char> = input.chars().collect();

        while self.idx < operators.len() {
            let op = operators[self.idx];

            match op {
                '>' => self.ptr += 1,
                '<' => self.ptr -= 1,
                '+' => self.mem[self.ptr] += 1,
                '-' => self.mem[self.ptr] -= 1,
                '[' => {
                    self.lst.push(self.idx);
                }
                ']' => {
                    if self.lst.len() == 0 {
                        panic!("Unexpected ']'");
                    } else {
                        if self.mem[self.ptr] == 0 {
                            self.lst.pop();
                        } else {
                            self.idx = *self.lst.last().unwrap()
                        }
                    }
                }
                ',' => {
                    self.mem[self.ptr] = std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .map(|byte| byte as u8)
                        .unwrap();
                }
                '.' => print!("{}", self.mem[self.ptr] as char),
                _ => {}
            }

            self.idx += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut program = Program::new();

    if args.len() >= 2 {
        program.run(&args[1]);
    } else {
        panic!("Not enough arguments");
    }
}
