use std::{collections::HashMap, io::{self, BufRead, Write}};

use crate::object::object::{CallInfo, Object};

fn print(args: Vec<Object>, _info: CallInfo) -> Object {
    let parts: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
    println!("{}", parts.join(" "));
    Object::Null
}

fn eprint(args: Vec<Object>, _info: CallInfo) -> Object {
    let parts: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
    eprintln!("{}", parts.join(" "));
    Object::Null
}

fn typeof_fn(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fmt.typeof takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    Object::StringType(args[0].type_name().to_string())
}

fn to_int(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fmt.to_int takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Integer(n) => Object::Integer(*n),
        Object::Float(f)   => Object::Integer(*f as i64),
        Object::Bool(b)    => Object::Integer(if *b { 1 } else { 0 }),
        Object::StringType(s) => match s.trim().parse::<i64>() {
            Ok(n) => Object::Integer(n),
            Err(_) => Object::Error { message: format!("fmt.to_int: cannot convert \"{}\" to integer", s), line: info.line, column: info.column },
        },
        _ => Object::Error { message: format!("fmt.to_int: cannot convert {} to integer", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn to_float(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fmt.to_float takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Float(f)   => Object::Float(*f),
        Object::Integer(n) => Object::Float(*n as f64),
        Object::StringType(s) => match s.trim().parse::<f64>() {
            Ok(f) => Object::Float(f),
            Err(_) => Object::Error { message: format!("fmt.to_float: cannot convert \"{}\" to float", s), line: info.line, column: info.column },
        },
        _ => Object::Error { message: format!("fmt.to_float: cannot convert {} to float", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn to_str(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fmt.to_str takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    Object::StringType(format!("{}", args[0]))
}

fn input(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fmt.input takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(prompt) => {
            print!("{}", prompt);
            io::stdout().flush().ok();
            let stdin = io::stdin();
            let mut line = String::new();
            match stdin.lock().read_line(&mut line) {
                Ok(_) => Object::StringType(line.trim_end_matches('\n').trim_end_matches('\r').to_string()),
                Err(e) => Object::Error { message: format!("fmt.input: {}", e), line: info.line, column: info.column },
            }
        }
        _ => Object::Error { message: format!("fmt.input expects STRING prompt, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn clear(args: Vec<Object>, info: CallInfo) -> Object {
    if !args.is_empty() {
        return Object::Error { message: "fmt.clear takes no arguments".to_string(), line: info.line, column: info.column };
    }
    if cfg!(windows) {
        std::process::Command::new("cmd").args(["/c", "cls"]).status().ok();
    } else {
        std::process::Command::new("clear").status().ok();
    }
    Object::Null
}

fn format_fn(args: Vec<Object>, info: CallInfo) -> Object {
    if args.is_empty() {
        return Object::Error { message: "fmt.format expects at least 1 argument".to_string(), line: info.line, column: info.column };
    }
    let template = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("fmt.format: first argument must be STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let mut result = String::new();
    let mut arg_idx = 1;
    let chars: Vec<char> = template.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '%' && i + 1 < chars.len() {
            match chars[i + 1] {
                '%' => { result.push('%'); i += 2; }
                's' | 'd' | 'f' => {
                    if arg_idx >= args.len() {
                        return Object::Error { message: format!("fmt.format: not enough arguments for format string"), line: info.line, column: info.column };
                    }
                    result.push_str(&format!("{}", args[arg_idx]));
                    arg_idx += 1;
                    i += 2;
                }
                _ => { result.push(chars[i]); i += 1; }
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    Object::StringType(result)
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("print".to_string(),   Object::Builtin(print));
    members.insert("eprint".to_string(),  Object::Builtin(eprint));
    members.insert("typeof".to_string(),  Object::Builtin(typeof_fn));
    members.insert("to_int".to_string(),  Object::Builtin(to_int));
    members.insert("to_float".to_string(), Object::Builtin(to_float));
    members.insert("to_str".to_string(),  Object::Builtin(to_str));
    members.insert("input".to_string(),   Object::Builtin(input));
    members.insert("clear".to_string(),   Object::Builtin(clear));
    members.insert("format".to_string(),  Object::Builtin(format_fn));
    Object::Module { name: "fmt".to_string(), pub_gated: false, members }
}
