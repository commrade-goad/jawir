use std::{env, fs};

#[derive(Debug)]
enum Token {
    Done,
    Start(String),
    Print(String),
    Call(String),
}

fn get_args(min: usize) -> Option<Vec<String>> {
    let user_args: Vec<String> = env::args().collect();
    if user_args.len() >= min {
        return Some(user_args);
    }
    return None;
}

fn gen_python_code(instruction: Vec<Token>) -> String {
    let mut in_func: bool = false;
    let mut ret: String = String::new();
    for idx in 0..instruction.len() {
        match &instruction[idx] {
            Token::Start(v) => {
                ret.push_str(&format!("def {}():\n", v));
                in_func = true;
            },
            Token::Print(v) => {
                if in_func {
                    ret.push_str(&format!("\t"));
                }
                ret.push_str(&format!("print(\"{}\")\n", v));
            }
            Token::Done => {
                in_func = false;
            }
            Token::Call(v) => {
                if in_func {
                    ret.push_str(&format!("\t"));
                }
                ret.push_str(&format!("{}\n", v));
            }
        }
    }
    return ret;
}

fn parse(content: &str) -> Vec<Token>{
    let mut instruction: Vec<Token> = Vec::new();

    let content_array: Vec<char> = content.chars().collect();
    let mut cursor = 0;
    let mut builder: String = String::new();
    let mut token: Token;
    'outer: while cursor < content_array.len() {
        builder.push_str(&content_array[cursor].to_string());
        // start
        let current_keyword: &str = "wiwitan";
        if builder.contains(current_keyword) {
            builder = "".to_string();
            cursor += 1;
            for _ in 0..128 {
                if content_array[cursor] != '(' {
                    builder.push_str(&content_array[cursor].to_string());
                } else {
                    cursor += 1;
                    break;
                }
                cursor += 1;

            }
            token = Token::Start(builder.clone().trim().to_string());
            instruction.push(token);
            builder = "".to_string();
            continue 'outer;
        }

        // print
        let current_keyword: &str = "nyetak";
        if builder.contains(current_keyword) {
            builder = "".to_string();
            cursor += 2;
            for _ in 0..1024 {
                if content_array[cursor] != ')' {
                    builder.push_str(&content_array[cursor].to_string());
                } else {
                    cursor += 1;
                    break;
                }
                cursor += 1;

            }
            token = Token::Print(builder.clone().trim().replace("\"", "").to_string());
            instruction.push(token);
            builder = "".to_string();
            continue 'outer;
        }

        // end
        let current_keyword: &str = "mari.";
        if builder.contains(current_keyword) {
            builder = "".to_string();
            token = Token::Done;
            instruction.push(token);
            continue 'outer;
        }

        // celuk
        let current_keyword: &str = "celuk";
        if builder.contains(current_keyword) {
            builder = "".to_string();
            cursor += 1;
            for _ in 0..1024 {
                if content_array[cursor] != ')' {
                    builder.push_str(&content_array[cursor].to_string());
                } else {
                    builder.push_str(&content_array[cursor].to_string());
                    cursor += 1;
                    break;
                }
                cursor += 1;

            }
            token = Token::Call(builder.clone().trim().to_string());
            instruction.push(token);
            builder = "".to_string();
            continue 'outer;
        }
        cursor += 1;

    }
    return instruction;
}

fn main() {
    let args: Vec<String> = match get_args(2) {
        Some(val) => val,
        None => {
            eprintln!("ERR: Not enought args!");
            std::process::exit(1);
        }
    };
    let file_path: &str = &args[1];
    let content: String = match fs::read_to_string(file_path) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("ERR: Failed to read the file with error -> {}", err);
            std::process::exit(2);
        }
    };
    let ints: Vec<Token> = parse(&content);
    let result:String = gen_python_code(ints);

    fs::write("output.py", result).unwrap();

}
