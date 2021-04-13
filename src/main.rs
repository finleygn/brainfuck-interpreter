use std::fs;
mod io;

pub struct Program {
    mem: [u8; 30000],
    ptr: usize,
    stack: Vec<usize>,
    index: usize,
}

impl Program {
    pub fn new() -> Program {
        Program {
            mem: [0u8; 30000], // Memory allocation
            ptr: 0,            // Current memory address
            index: 0,          // Current index to be interpreted
            stack: vec![],     // Loop stack start indexes
        }
    }

    fn find_closing(&self, operators: &[char]) -> Result<usize, ()> {
        let mut loop_count = 1;
        let mut index = 0;

        for inner_op in operators {
            if inner_op == &'[' {
                loop_count += 1;
            } else if inner_op == &']' {
                loop_count -= 1;
            }
            if loop_count == 0 {
                return Ok(index);
            }

            index += 1;
        }

        Err(())
    }

    pub fn run(&mut self, input: &str) {
        let operators: Vec<char> = input.chars().collect();

        while self.index < operators.len() {
            let op = operators[self.index];

            match op {
                '>' => self.ptr += 1,
                '<' => self.ptr -= 1,
                '+' => self.mem[self.ptr] = self.mem[self.ptr].wrapping_add(1),
                '-' => self.mem[self.ptr] = self.mem[self.ptr].wrapping_sub(1),
                '[' => {
                    self.stack.push(self.index);

                    if self.mem[self.ptr] == 0 {
                        match self.find_closing(&operators[self.index + 1..]) {
                            Ok(index) => self.index += index,
                            Err(_) => panic!("No closing [ for char at {}", self.index),
                        }
                    }
                }
                ']' => {
                    if self.stack.len() == 0 {
                        panic!("Unexpected ']'");
                    } else {
                        if self.mem[self.ptr] == 0 {
                            self.stack.pop();
                        } else {
                            self.index = *self.stack.last().unwrap()
                        }
                    }
                }
                ',' => {
                    self.mem[self.ptr] = io::read().unwrap();
                }
                '.' => io::write(self.mem[self.ptr]),
                _ => {}
            }

            self.index += 1;
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
