use std::collections::HashMap;

use crate::object::object::{CallInfo, Object};

fn get_env(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "os.get_env takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(key) => Object::StringType(std::env::var(key).unwrap_or_default()),
        _ => Object::Error { message: format!("os.get_env expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn set_env(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "os.set_env takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let key = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("os.set_env: key must be STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let val = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("os.set_env: value must be STRING, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    unsafe { std::env::set_var(&key, &val) };
    Object::Bool(true)
}

fn get_wd(args: Vec<Object>, info: CallInfo) -> Object {
    if !args.is_empty() {
        return Object::Error { message: "os.get_wd takes no arguments".to_string(), line: info.line, column: info.column };
    }
    match std::env::current_dir() {
        Ok(p) => Object::StringType(p.to_string_lossy().to_string()),
        Err(e) => Object::Error { message: format!("os.get_wd: {}", e), line: info.line, column: info.column },
    }
}

fn exit(args: Vec<Object>, _info: CallInfo) -> Object {
    let code = match args.first() {
        Some(Object::Integer(n)) => *n as i32,
        _ => 0,
    };
    std::process::exit(code);
}

fn hostname(args: Vec<Object>, info: CallInfo) -> Object {
    if !args.is_empty() {
        return Object::Error { message: "os.hostname takes no arguments".to_string(), line: info.line, column: info.column };
    }
    match std::process::Command::new("hostname").output() {
        Ok(out) => Object::StringType(String::from_utf8_lossy(&out.stdout).trim().to_string()),
        Err(e) => Object::Error { message: format!("os.hostname: {}", e), line: info.line, column: info.column },
    }
}

pub fn module() -> Object {
    let args: Vec<Object> = std::env::args().map(|a| Object::StringType(a)).collect();

    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("args".to_string(),     Object::Array(args));
    members.insert("platform".to_string(), Object::StringType(std::env::consts::OS.to_string()));
    members.insert("arch".to_string(),     Object::StringType(std::env::consts::ARCH.to_string()));
    members.insert("get_env".to_string(),  Object::Builtin(get_env));
    members.insert("set_env".to_string(),  Object::Builtin(set_env));
    members.insert("get_wd".to_string(),   Object::Builtin(get_wd));
    members.insert("exit".to_string(),     Object::Builtin(exit));
    members.insert("hostname".to_string(), Object::Builtin(hostname));
    Object::Module { members }
}
