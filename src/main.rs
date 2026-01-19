use std::fmt::Write as FmtWrite;

mod app;

fn main() {
    let mut stack = Stack::new();
    let input = "10 5 + .s \"hello world\" .";
    let tokens = parse(input);

    if let Err(e) = execute(tokens, &mut stack) {
        stack.write_output(&format!("Error: {:?}", e));
    }

    let output = stack.get_output().to_string();
    let _ = app::main_rat(output);
}

#[derive(Debug)]
enum StackValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug)]
enum Token {
    Number(f64),
    String(String),
    Word(String),
    Boolean(bool),
}

#[derive(Debug)]
enum StackError {
    Underflow,
    TypeMismatch,
    UnknownWord,
}

struct Stack {
    stack_values: Vec<StackValue>,
    output: String,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            stack_values: Vec::new(),
            output: String::new(),
        }
    }

    fn push(&mut self, value: StackValue) {
        self.stack_values.push(value);
    }

    fn pop(&mut self) -> Result<StackValue, StackError> {
        self.stack_values.pop().ok_or(StackError::Underflow)
    }

    fn peek(&self) -> Result<&StackValue, StackError> {
        self.stack_values.last().ok_or(StackError::Underflow)
    }

    fn write_output(&mut self, text: &str) {
        writeln!(&mut self.output, "{}", text).unwrap();
    }

    fn get_output(&self) -> &str {
        &self.output
    }
}

fn parse(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            continue;
        }

        if ch == '"' {
            // Handle quoted string - collect until closing "
            let mut string_content = String::new();

            while let Some(next_ch) = chars.next() {
                if next_ch == '"' {
                    break; // Found closing quote
                }
                string_content.push(next_ch);
            }
            tokens.push(Token::String(string_content));
        } else {
            // Collect characters until whitespace - this is a word or number
            let mut word = String::new();
            word.push(ch);

            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_whitespace() {
                    break;
                }
                word.push(chars.next().unwrap());
            }

            // Check type of token
            if let Ok(num) = word.parse::<f64>() {
                tokens.push(Token::Number(num));
            } else if let Ok(bool) = word.parse::<bool>() {
                tokens.push(Token::Boolean(bool));
            } else {
                tokens.push(Token::Word(word));
            }
        }
    }
    tokens
}

fn execute(tokens: Vec<Token>, stack: &mut Stack) -> Result<(), StackError> {
    for token in tokens {
        match token {
            Token::Number(n) => stack.push(StackValue::Number(n)),
            Token::String(s) => stack.push(StackValue::String(s)),
            Token::Boolean(b) => stack.push(StackValue::Boolean(b)),
            Token::Word(w) => match w.as_str() {
                "." => {
                    let output = match stack.pop() {
                        Ok(StackValue::Number(n)) => n.to_string(),
                        Ok(StackValue::String(s)) => s.to_string(),
                        Ok(StackValue::Boolean(b)) => b.to_string(),
                        Err(e) => format!("Error: {:?}", e),
                    };
                    stack.write_output(&output);
                }
                ".s" => {
                    let output = match stack.peek() {
                        Ok(StackValue::Number(n)) => n.to_string(),
                        Ok(StackValue::String(s)) => s.to_string(),
                        Ok(StackValue::Boolean(b)) => b.to_string(),
                        Err(e) => format!("Error: {:?}", e),
                    };
                    stack.write_output(&output);
                }
                "+" => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;

                    match (a, b) {
                        (StackValue::Number(n1), StackValue::Number(n2)) => {
                            stack.push(StackValue::Number(n1 + n2));
                        }
                        _ => {
                            return Err(StackError::TypeMismatch);
                        }
                    }
                }
                "-" => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;

                    match (a, b) {
                        (StackValue::Number(n1), StackValue::Number(n2)) => {
                            stack.push(StackValue::Number(n1 - n2));
                        }
                        _ => {
                            return Err(StackError::TypeMismatch);
                        }
                    }
                }
                "*" => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;

                    match (a, b) {
                        (StackValue::Number(n1), StackValue::Number(n2)) => {
                            stack.push(StackValue::Number(n1 * n2));
                        }
                        _ => {
                            return Err(StackError::TypeMismatch);
                        }
                    }
                }
                "/" => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;

                    match (a, b) {
                        (StackValue::Number(n1), StackValue::Number(n2)) => {
                            stack.push(StackValue::Number(n1 / n2));
                        }
                        _ => {
                            return Err(StackError::TypeMismatch);
                        }
                    }
                }
                _ => return Err(StackError::UnknownWord),
            },
        }
    }
    Ok(())
}
