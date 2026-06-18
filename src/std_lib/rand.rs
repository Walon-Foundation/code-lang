use std::collections::HashMap;

use rand::Rng;

use crate::object::object::{CallInfo, Object};

fn int(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "rand.int takes 2 arguments: min and max (inclusive)".to_string(), line: info.line, column: info.column };
    }
    let min = match &args[0] {
        Object::Integer(n) => *n,
        _ => return Object::Error { message: format!("rand.int expects INTEGER as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let max = match &args[1] {
        Object::Integer(n) => *n,
        _ => return Object::Error { message: format!("rand.int expects INTEGER as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    if min > max {
        return Object::Error { message: format!("rand.int: min ({}) must be <= max ({})", min, max), line: info.line, column: info.column };
    }
    Object::Integer(rand::rng().random_range(min..=max))
}

fn float(args: Vec<Object>, info: CallInfo) -> Object {
    if !args.is_empty() {
        return Object::Error { message: "rand.float takes no arguments".to_string(), line: info.line, column: info.column };
    }
    Object::Float(rand::rng().random::<f64>())
}

fn choice(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "rand.choice takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => {
            if elems.is_empty() { return Object::Null; }
            let i = rand::rng().random_range(0..elems.len());
            elems[i].clone()
        }
        _ => Object::Error { message: format!("rand.choice expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn shuffle(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "rand.shuffle takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => {
            let mut new = elems.clone();
            let len = new.len();
            let mut rng = rand::rng();
            for i in (1..len).rev() {
                let j = rng.random_range(0..=i);
                new.swap(i, j);
            }
            Object::Array(new)
        }
        _ => Object::Error { message: format!("rand.shuffle expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("int".to_string(),     Object::Builtin(int));
    members.insert("float".to_string(),   Object::Builtin(float));
    members.insert("choice".to_string(),  Object::Builtin(choice));
    members.insert("shuffle".to_string(), Object::Builtin(shuffle));
    Object::Module { name: "rand".to_string(), pub_gated: false, members }
}
