use text_io::read;

enum TokenType {
    Digit,
    Operator,
    Variable,
}

struct Token {
    token_type: TokenType, //the type of token, determines which properties are relevant
    num_str: String, //holds the string representation of the value
    op_val: char, //the operator value of the token (if applicable)
    f_val: f32, //the final, numeric value of the token (if applicable)
}

struct Variable {
    name: String, //the name of the variable, as defined by the user
    f_val: f32, //the final, numeric value of the variable
}

fn main() {
    println!("CALCULATOR");

    let mut vars: Vec<Variable> = vec![];
    let mut ans: f32 = 0.0;

    loop {
        //get user input
        let user_text: String = read!("{}\n");

        let tokens = parse_line(user_text);

        //calculate final output and print
        let output = calculate(tokens, vars, ans);
        let result = output.0;
        vars = output.1;
        println!("{}", result);
        ans = result; //update ans for next time
    }
}

fn parse_line(input: String) -> Vec<Token> {

    //get chars from input
    let mut chars = input.chars();
    //setup tokens vec
    let mut tokens: Vec<Token> = vec![];

    'line_loop: loop {
        let c_option = chars.next(); //char result

        if c_option == None {
            break 'line_loop; //stop looping line if no next char
        }

        let c: char = c_option.unwrap(); //char value of next char

        match c {
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' => { //digit
                let char_str = c.to_string();
                let mut push_new_token = false;

                if tokens.len() == 0 {
                    push_new_token = true; //no preceding tokens, must be new
                }
                else {
                    let last_index = tokens.len() - 1; //index of last token

                    match tokens[last_index].token_type { //check if preceding token is digit
                        TokenType::Digit => { //it is
                            //append current digit to previous, recalculate f_val
                            let new_num_str: String = format!("{}{}", &tokens[last_index].num_str, char_str);
                            let new_f_val: f32 = (&new_num_str).parse::<f32>().unwrap();
                            //apply new values
                            tokens[last_index].num_str = new_num_str;
                            tokens[last_index].f_val = new_f_val;
                        },
                        _ => push_new_token = true, //isn't digit, push a new token
                    }

                }

                if push_new_token { //add a new token
                    let new_f_val: f32 = (&char_str).parse::<f32>().unwrap(); //get f_val from current char string
                    //push new token
                    tokens.push(Token {
                        token_type: TokenType::Digit,
                        num_str: char_str,
                        op_val: 'd', //no operator value, its a digit
                        f_val: new_f_val,
                    });
                }
            }
            '.' => { //decimal
                if tokens.len() > 0 { //only apply a decimal if theres a preceding digit
                    let char_str = c.to_string();
                    let last_index = tokens.len() - 1;

                    match tokens[last_index].token_type {
                        TokenType::Digit => { //there is a preceding digit
                            //add decimal to num_str and recalculate f_val
                            let new_num_str: String = format!("{}{}", &tokens[last_index].num_str, char_str);
                            let new_f_val: f32 = (&new_num_str).parse::<f32>().unwrap();
                            //apply changes
                            tokens[last_index].num_str = new_num_str;
                            tokens[last_index].f_val = new_f_val;
                        }
                        _ => {}
                    }
                }
            }
            '+'|'-'|'*'|'/'|'=' => { //operator
                tokens.push(Token {
                    token_type: TokenType::Operator,
                    num_str: String::from("0"), //no numeric value
                    op_val: c, //get operator value from char
                    f_val: 0.0, //no f_val
                });
            }
            'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'|'i'|'j'|'k'|'l'|'m'|'n'|'o'|'p'|'q'|'r'|'s'|'t'|'u'|'v'|'w'|'x'|'y'|'z'|
            'A'|'B'|'C'|'D'|'E'|'F'|'G'|'H'|'I'|'J'|'K'|'L'|'M'|'N'|'O'|'P'|'Q'|'R'|'S'|'T'|'U'|'V'|'W'|'X'|'Y'|'Z'|
            '_' => { //alpha (variable naming)
                let char_str = c.to_string();
                let mut push_new_token = false;

                if tokens.len() == 0 {
                    push_new_token = true;
                }
                else {
                    let last_index = tokens.len() - 1;

                    match tokens[last_index].token_type {
                        TokenType::Variable => {
                            let new_num_str: String = format!("{}{}", &tokens[last_index].num_str, char_str);
                            tokens[last_index].num_str = new_num_str;
                        }
                        _ => {
                            push_new_token = true;
                        }
                    }
                }

                if push_new_token {
                    tokens.push(Token {
                        token_type: TokenType::Variable,
                        num_str: char_str,
                        op_val: 'v',
                        f_val: 0.0,
                    })
                }

            }
            '\n'|'\r'|' ' => {} //just ignore these guys
            _ => { //invalid character, warn & break
                println!("invalid character: {}", c);
                break;
            }, //anything else
        }
    }

    tokens //return tokens
}

fn calculate(mut tokens: Vec<Token>, mut vars: Vec<Variable>, ans: f32) -> (f32, Vec<Variable>) {
    //establish initial value for calculations
    let mut value: f32 = 0.0;

    //populate tokens with variable values
    tokens = populate_var_tokens(tokens, &vars, ans);

    //loop through all tokens
    for i in 0..(tokens.len() - 1) {
        match tokens[i].token_type {
            TokenType::Digit => { //its a digit
                if i == 0 { value = tokens[i].f_val }; //assign starting value if its the first token
            }
            TokenType::Operator => { //its an operator, respond accordingly
                if tokens[i].op_val == '+' { //add next token f_val
                    value += tokens[i + 1].f_val;
                }
                if tokens[i].op_val == '-' { //subtract next token f_val
                    value -= tokens[i + 1].f_val;
                }
                if tokens[i].op_val == '*' { //multiply with next token f_val
                    value *= tokens[i + 1].f_val;
                }
                if tokens[i].op_val == '/' { //divide by next token f_val
                    value /= tokens[i + 1].f_val;
                }
                if tokens[i].op_val == '=' {
                    if i > 0 {
                        let var_token = &tokens[i - 1];
                        match var_token.token_type {
                            TokenType::Variable => {
                                //the value to set the variable to
                                let set_val = tokens[i + 1].f_val;

                                if var_exists(&vars, &var_token.num_str) { //variable already exists!
                                    //find variable and set the value
                                    for k in 0..vars.len() {
                                        if vars[k].name == var_token.num_str { //found one with matching name
                                            vars[k].f_val = set_val; //set value accordingly
                                        }
                                    }
                                }
                                else { //new variable, create it!
                                    vars.push(Variable {
                                        name: String::from(&var_token.num_str),
                                        f_val: set_val,
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            TokenType::Variable => { //its a variable
                if i == 0 { value = tokens[i].f_val; }
            }
            _ => {}
        }
    }

    (value, vars) //return value
}

fn populate_var_tokens(mut tokens: Vec<Token>, vars: &Vec<Variable>, ans: f32) -> Vec<Token> {
    //loop through all tokens and fill in variable values
    for i in 0..(tokens.len()) {
        match tokens[i].token_type {
            TokenType::Variable => { //its a variable

                let var_name: String = String::from(&tokens[i].num_str);

                //populate based on variable name (checking reserved first)
                if var_name == String::from("ANS") { //ANS
                    tokens[i].f_val = ans;
                }
                else if var_name == String::from("PI") { //PI
                    tokens[i].f_val = 3.14;
                }
                else { //it isn't reserved, so check against var list
                    if var_exists(&vars, &var_name) { //variable exists
                        //find the variable and assign token value accordingly
                        for k in 0..vars.len() {
                            if vars[k].name == var_name { //found one with matching name
                                tokens[i].f_val = vars[k].f_val;
                            }
                        }
                    }
                    else { //variable does not exist
                        tokens[i].f_val = 0.0; //assign default value (0.0)
                    }
                }
            }
            _ => {}
        }
    }

    tokens //return tokens
}

fn var_exists(vars: &Vec<Variable>, name: &str) -> bool {
    //loop through vars, check if one of them has a name that matches
    for i in 0..vars.len() {
        if vars[i].name == name {
            return true; //found one, return!
        }
    }

    false //none found, return false
}

#[allow(dead_code)]
fn print_token(t: &Token) {
    println!("TOKEN ( val:{num_str} op:{op_val} num:{f_val} )", num_str = t.num_str, op_val = t.op_val, f_val = t.f_val);
}