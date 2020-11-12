use text_io::read;
use rand::prelude::*;

enum TokenType {
    Digit,
    Operator,
    Variable,
    Function,
    GroupMark,
    ExprEnd,
    SimpleValue,
}

struct Token {
    token_type: TokenType, //the type of token, determines which properties are relevant
    val_str: String, //holds the string representation of the value
    op_val: char, //the operator value of the token (if applicable)
    f_val: f32, //the final, numeric value of the token (if applicable)
    priority: i32, //the number of groupings deep the token occurs
}

struct Variable {
    name: String, //the name of the variable, as defined by the user
    f_val: f32, //the final, numeric value of the variable
}

struct Function {
    name: String, //the name of the function, as defined by the user (should begin with '@')
    expression: String, //the string to evaluate when the function is called
}

fn main() {
    println!("calc-rs");

    let mut vars: Vec<Variable> = vec![];
    let mut fns: Vec<Function> = vec![];
    let mut ans: f32 = 0.0;

    loop {
        //get user input
        let user_text: String = read!("{}\n");

        let tokens = parse_line(user_text);

        //calculate final output and print
        let output = calculate(tokens, vars, fns, ans);
        let result = output.0;
        vars = output.1;
        fns = output.2;
        println!("= {}", result);
        ans = result; //update ans for next time
    }
}

fn parse_line(input: String) -> Vec<Token> {

    //get chars from input
    let mut chars = input.chars();
    //setup tokens vec
    let mut tokens: Vec<Token> = vec![];
    let mut open_groups = 0;

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
                            let new_num_str: String = format!("{}{}", &tokens[last_index].val_str, char_str);
                            let new_f_val: f32 = (&new_num_str).parse::<f32>().unwrap();
                            //apply new values
                            tokens[last_index].val_str = new_num_str;
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
                        val_str: char_str,
                        op_val: 'd', //no operator value, its a digit
                        f_val: new_f_val,
                        priority: open_groups * 2,
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
                            let new_num_str: String = format!("{}{}", &tokens[last_index].val_str, char_str);
                            let new_f_val: f32 = (&new_num_str).parse::<f32>().unwrap();
                            //apply changes
                            tokens[last_index].val_str = new_num_str;
                            tokens[last_index].f_val = new_f_val;
                        }
                        _ => {}
                    }
                }
            }
            '+'|'-'|'*'|'/'|'='|'^'|'>'|'<'|':' => { //operator
                let mut op_priority = open_groups * 2;
                if c == '*' || c == '/' { //give slight priority to multiplication & division (pemdas!)
                    op_priority += 1;
                }
                if c == ':' { //give significant priority to assignment
                    op_priority += 2;
                }
                tokens.push(Token {
                    token_type: TokenType::Operator,
                    val_str: String::from(c), //no numeric value
                    op_val: c, //get operator value from char
                    f_val: 0.0, //no f_val
                    priority: op_priority,
                });
            }
            'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'|'i'|'j'|'k'|'l'|'m'|'n'|'o'|'p'|'q'|'r'|'s'|'t'|'u'|'v'|'w'|'x'|'y'|'z'|
            'A'|'B'|'C'|'D'|'E'|'F'|'G'|'H'|'I'|'J'|'K'|'L'|'M'|'N'|'O'|'P'|'Q'|'R'|'S'|'T'|'U'|'V'|'W'|'X'|'Y'|'Z'|
            '_'|'@' => { //alpha + symbols (variable naming)
                let char_str = c.to_string();
                let mut push_new_token = false;

                if tokens.len() == 0 {
                    push_new_token = true;
                }
                else {
                    let last_index = tokens.len() - 1;

                    match tokens[last_index].token_type {
                        TokenType::Variable => {
                            let new_num_str: String = format!("{}{}", &tokens[last_index].val_str, char_str);
                            tokens[last_index].val_str = new_num_str;
                        }
                        _ => {
                            push_new_token = true;
                        }
                    }
                }

                if push_new_token {
                    tokens.push(Token {
                        token_type: TokenType::Variable,
                        val_str: char_str,
                        op_val: 'v',
                        f_val: 0.0,
                        priority: open_groups * 2,
                    })
                }

            }
            '(' => { //open group
                open_groups += 1;
                tokens.push(Token {
                    token_type: TokenType::GroupMark,
                    val_str: String::from('('),
                    op_val: '(',
                    f_val: 0.0,
                    priority: open_groups * 2,
                });
            }
            ')' => { //close group
                open_groups -= 1;
                tokens.push(Token {
                    token_type: TokenType::GroupMark,
                    val_str: String::from(')'),
                    op_val: ')',
                    f_val: 0.0,
                    priority: open_groups * 2,
                });
            }
            '}' => { //expression end
                tokens.push(Token {
                    token_type: TokenType::ExprEnd,
                    val_str: String::from("}"),
                    op_val: '}',
                    f_val: 0.0,
                    priority: open_groups * 2,
                })
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

fn calculate(mut tokens: Vec<Token>, mut vars: Vec<Variable>, mut fns: Vec<Function>, ans: f32) -> (f32, Vec<Variable>, Vec<Function>) {

    //populate tokens with variable values
    let (pop_tokens, pop_fns) = populate_var_tokens(tokens, &vars, fns, ans);
    tokens = pop_tokens;
    fns = pop_fns;

    let mut consume_indexes: Vec<usize> = vec![];
    let mut combine_surrounding = true;

    //loop through tokens until an operator cannot be found
    while get_highest_priority_operator(&tokens) > 0 {

        let op_i = get_highest_priority_operator(&tokens) as usize; //find highest priority value

        //get useful values for calculation
        let op_c = tokens[op_i].op_val;
        let left_token = &tokens[op_i - 1];
        let right_token = &tokens[op_i + 1];
        let left_value: f32 = left_token.f_val;
        let right_value: f32 = right_token.f_val;

        //calculate combined values
        let mut combined: f32 = 0.0;
        if op_c == '+' { combined = left_value + right_value } //addition
        if op_c == '-' { combined = left_value - right_value } //subtraction
        if op_c == '*' { combined = left_value * right_value } //multiplication
        if op_c == '/' { combined = left_value / right_value } //division
        if op_c == '^' { combined = left_value.powf(right_value) } //exponent
        if op_c == ':' { //assignment
            match left_token.token_type {
                TokenType::Variable => { //check if left token is variable
                    let set_val = right_value;
                    let var_i = get_var_index(&vars, &left_token.val_str);

                    if var_i > -1 { //var has index, set value
                        vars[var_i as usize].f_val = set_val;
                    }
                    else { //var doesn't exist, create it
                        vars.push(Variable {
                            name: String::from(&left_token.val_str),
                            f_val: set_val,
                        })
                    }
                    combined = set_val; //set combined val so it outputs
                }
                TokenType::Function => { //check if left token is function
                    //get expression of function
                    let mut fn_expression: String = String::from(""); //prepare empty string for expression

                    //loop through next tokens, add them to expression until a ExprEnd is encountered
                    for k in (op_i + 1)..tokens.len() {

                        match tokens[k].token_type {
                            TokenType::ExprEnd => { //expression end, stop loop
                                consume_indexes.push(k); //remember to consume
                                break;
                            }
                            _ => { //not expression end, so add to function expression string
                                let next_token_content = &tokens[k].val_str;
                                fn_expression = format!("{}{}", fn_expression, next_token_content);
                                consume_indexes.push(k); //remember to consume
                            }
                        }
                    }

                    let fn_i = get_fn_index(&fns, &left_token.val_str);
                    if fn_i > -1 { //fn has index, assign expression
                        fns[fn_i as usize].expression = fn_expression; //assign fn expression
                    }
                    else { //fn doesn't exist, push it
                        fns.push(Function {
                            name: String::from(&left_token.val_str),
                            expression: fn_expression,
                        })
                    }

                    //fill in fake combined value, prevent combining
                    combine_surrounding = false;
                    combined = 0.0;
                }
                _ => {}
            }
        }
        if op_c == '=' { //equals
            if left_value == right_value { combined = 1.0; }
            else { combined = 0.0; }
        }
        if op_c == '>' { //greater than
            if left_value > right_value { combined = 1.0; }
            else { combined = 0.0; }
        }
        if op_c == '<' { //less than
            if left_value < right_value { combined = 1.0; }
            else { combined = 0.0; }
        }

        //add simplevalue token
        tokens[op_i] = Token {
            token_type: TokenType::SimpleValue,
            val_str: combined.to_string(),
            op_val: 's',
            f_val: combined,
            priority: 0,
        };

        //remove surrounding tokens
        if combine_surrounding {
            if op_i - 1 < tokens.len() {
                tokens.remove(op_i - 1);
            }
            if op_i < tokens.len() {
                tokens.remove(op_i);
            }
        }

    }

    let result = tokens[0].f_val; //set result to remaining simplevalue

    (result, vars, fns) //return value
}

fn populate_var_tokens(mut tokens: Vec<Token>, vars: &Vec<Variable>, mut fns: Vec<Function>, ans: f32) -> (Vec<Token>, Vec<Function>) {
    //loop through all tokens and fill in variable values
    for i in 0..(tokens.len()) {
        match tokens[i].token_type {
            TokenType::Variable => { //its a variable

                let var_name: String = String::from(&tokens[i].val_str);

                if var_name.starts_with("@") { //its a function
                    tokens[i].token_type = TokenType::Function; //update type of token

                    //populate f_val by evaluating function
                    //but first: take action if its a reserved name
                    if var_name == String::from("@FN") { //print current list of functions
                        for k in 0..fns.len() { print_function(&fns[k]); }
                    }
                    else if var_name == String::from("@VAR") { //print current list of variables
                        for k in 0..vars.len() { print_variable(&vars[k]); }
                    }
                    else { //it isn't reserved, so evaluate and return
                        //get index of function
                        let fn_i: i32 = get_fn_index(&fns, &var_name);
                        let mut fn_val: f32 = 0.0;
                        
                        if fn_i > -1 { //function exists, populate value by evaluating it
                            let (fn_eval_out, new_fns) = evaluate_function(fns, fn_i as usize, vec![], ans);
                            fn_val = fn_eval_out; //update val with eval output
                            fns = new_fns; //update fns
                        }

                        //otherwise, sub in 0 for value
                        //(it is likely that this function is about to be assigned to)
                        tokens[i].f_val = fn_val;
                    }
                }
                else { //its just a normal variable
                    //populate f_val based on variable name
                    //but first: fill value manually if its a reserved name
                    if var_name == String::from("ANS") { //ANS
                        tokens[i].f_val = ans;
                    }
                    else if var_name == String::from("PI") { //PI
                        tokens[i].f_val = 3.14;
                    }
                    else if var_name == String::from("RAND") { //random float
                        tokens[i].f_val = thread_rng().gen();
                    }
                    else if var_name == String::from("_") { //reserved as empty
                        tokens[i].op_val = '_';
                        tokens[i].f_val = 0.0;
                    }
                    else { //it isn't reserved, so check against var list
                        let var_i = get_var_index(&vars, &var_name);

                        if var_i > -1 { //variable exists
                            tokens[i].f_val = vars[var_i as usize].f_val;
                        }
                        else { //variable doesn't exist, placeholder of 0.0
                            tokens[i].f_val = 0.0;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    (tokens, fns) //return tokens
}

fn get_var_index(vars: &Vec<Variable>, name: &str) -> i32 {
    //loop through vars, check if one of them has a name that matches
    for i in 0..vars.len() {
        if vars[i].name == name {
            return i as i32; //found one, return!
        }
    }

    -1 //none found, return false
}

fn get_fn_index(fns: &Vec<Function>, name: &str) -> i32 {
    //loop through functions, check if one of them has a name that matches
    for i in 0..fns.len() {
        if fns[i].name == name {
            return i as i32; //found it, return index
        }
    }
    -1 //none found, return false
}

fn get_highest_priority_operator(tokens: &Vec<Token>) -> i32 {
    
    let mut highest_priority = -1;
    let mut highest_priority_index: i32 = 0;

    for i in 0..tokens.len() {
        match tokens[i].token_type {
            TokenType::Operator => {
                if tokens[i].priority > highest_priority {
                    highest_priority = tokens[i].priority;
                    highest_priority_index = i as i32;
                }
            }
            _ => {}
        }
    }

    highest_priority_index //return final index

}


fn evaluate_function(fns: Vec<Function>, fn_i: usize, fn_vars: Vec<Variable>, ans: f32) -> (f32, Vec<Function>) {
    let expression = String::from(&fns[fn_i].expression); //load expression
    let tokens = parse_line(expression); //parse tokens from expression

    let (output_val, _new_vars, new_fns) = calculate(tokens, fn_vars, fns, ans); //evaluate as you would any other string of tokens
    (output_val, new_fns) //return calculation (ignore variables)
}

fn print_function(function: &Function) {
    println!("FN: {}, {}", function.name, function.expression);
}

fn print_variable(variable: &Variable) {
    println!("VAR: {}, {}", variable.name, variable.f_val);
}

#[allow(dead_code)]
fn print_token(t: &Token) {
    println!("TOKEN ( val:{num_str} op:{op_val} num:{f_val} )", num_str = t.val_str, op_val = t.op_val, f_val = t.f_val);
}