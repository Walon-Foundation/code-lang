use std::collections::HashMap;

use crate::object::object::Object;

fn first(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.first expects 1 argument".to_string(), line: 0, column: 0 };
    }
    match &args[0] {
        Object::Array(elems) => elems.first().cloned().unwrap_or(Object::Null),
        _ => Object::Error { message: format!("arrays.first expects ARRAY, got {}", args[0].type_name()), line: 0, column: 0 },
    }
}

fn last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.last expects 1 argument".to_string(), line: 0, column: 0 };
    }
    match &args[0] {
        Object::Array(elems) => elems.last().cloned().unwrap_or(Object::Null),
        _ => Object::Error { message: format!("arrays.last expects ARRAY, got {}", args[0].type_name()), line: 0, column: 0 },
    }
}

fn rest(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.rest expects 1 argument".to_string(), line: 0, column: 0 };
    }
    match &args[0] {
        Object::Array(elems) => {
            if elems.is_empty() { return Object::Null; }
            Object::Array(elems[1..].to_vec())
        }
        _ => Object::Error { message: format!("arrays.rest expects ARRAY, got {}", args[0].type_name()), line: 0, column: 0 },
    }
}

fn push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.push expects 2 arguments".to_string(), line: 0, column: 0 };
    }
    match &args[0] {
        Object::Array(elems) => {
            let mut new = elems.clone();
            new.push(args[1].clone());
            Object::Array(new)
        }
        _ => Object::Error { message: format!("arrays.push expects ARRAY as first argument, got {}", args[0].type_name()), line: 0, column: 0 },
    }
}

fn len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.len expects 1 argument".to_string(), line: 0, column: 0 };
    }
    match &args[0] {
        Object::Array(elems) => Object::Integer(elems.len() as i64),
        Object::StringType(s) => Object::Integer(s.len() as i64),
        _ => Object::Error { message: format!("arrays.len expects ARRAY or STRING, got {}", args[0].type_name()), line: 0, column: 0 },
    }
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("first".to_string(), Object::Builtin(first));
    members.insert("last".to_string(),  Object::Builtin(last));
    members.insert("rest".to_string(),  Object::Builtin(rest));
    members.insert("push".to_string(),  Object::Builtin(push));
    members.insert("len".to_string(),   Object::Builtin(len));
    Object::Module { members }
}
