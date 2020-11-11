use text_io::read;

enum TokenType {
    Digit,
    Operator,
}

struct Token {
    token_type: TokenType,
    num_str: String,
    op_val: char,
    f_val: f32,
}

fn main() {
    println!("CALCULATOR");

    let user_text: String = read!("{}\n");
    let mut chars = user_text.chars();

    let mut tokens: Vec<Token> = vec![];

    loop {
        let c_option = chars.next();

        if c_option == None {
            break;
        }

        let c: char = c_option.unwrap();

        match c {
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' => { //digit
                let char_str = c.to_string();
                let mut push_new_token = false;

                if tokens.len() == 0 {
                    push_new_token = true; 
                }
                else {
                    let last_index = tokens.len() - 1;

                    match tokens[last_index].token_type {
                        TokenType::Digit => {
                            let new_num_str: String = format!("{}{}", &tokens[last_index].num_str, char_str);
                            let new_f_val: f32 = (&new_num_str).parse::<f32>().unwrap();
                            tokens[last_index].num_str = new_num_str;
                            tokens[last_index].f_val = new_f_val;
                        },
                        _ => push_new_token = true,
                    }

                }

                if push_new_token {
                    let new_f_val: f32 = (&char_str).parse::<f32>().unwrap();
                    tokens.push(Token {
                        token_type: TokenType::Digit,
                        num_str: char_str,
                        op_val: 'd',
                        f_val: new_f_val,
                    });
                }
            }
            '.' => { //decimal
                if tokens.len() > 0 {
                    let char_str = c.to_string();
                    let last_index = tokens.len() - 1;

                    match tokens[last_index].token_type {
                        TokenType::Digit => {
                            let new_num_str: String = format!("{}{}", &tokens[last_index].num_str, char_str);
                            let new_f_val: f32 = (&new_num_str).parse::<f32>().unwrap();
                            tokens[last_index].num_str = new_num_str;
                            tokens[last_index].f_val = new_f_val;
                        }
                        _ => {}
                    }
                }
            }
            '+'|'-'|'*'|'/' => { //operator
                tokens.push(Token {
                    token_type: TokenType::Operator,
                    num_str: String::from("0"),
                    op_val: c,
                    f_val: 0.0,
                });
            },
            '\n'|'\r'|' ' => {} //just ignore these guys
            _ => {
                println!("invalid character: {}", c);
                break;
            }, //anything else
        }
    }

    let mut value: f32 = 0.0;

    for i in 0..(tokens.len() - 1) {
        match tokens[i].token_type {
            TokenType::Digit => {
                if i == 0 { value = tokens[i].f_val };
            }
            TokenType::Operator => {
                if tokens[i].op_val == '+' {
                    value += tokens[i + 1].f_val;
                }
                if tokens[i].op_val == '-' {
                    value -= tokens[i + 1].f_val;
                }
                if tokens[i].op_val == '*' {
                    value *= tokens[i + 1].f_val;
                }
                if tokens[i].op_val == '/' {
                    value /= tokens[i + 1].f_val;
                }
            }
        }
    }

    println!("Result: {}", value);

}

#[allow(dead_code)]
fn print_token(t: &Token) {
    println!("t {num_str} {op_val} {f_val}", num_str = t.num_str, op_val = t.op_val, f_val = t.f_val);
}