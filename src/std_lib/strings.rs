use std::collections::HashMap;

use crate::object::object::{CallInfo, Object};

fn to_upper(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.to_upper takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => Object::StringType(s.to_uppercase()),
        _ => Object::Error { message: format!("strings.to_upper expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn to_lower(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.to_lower takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => Object::StringType(s.to_lowercase()),
        _ => Object::Error { message: format!("strings.to_lower expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn split(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "strings.split takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let s = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.split expects STRING as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let sep = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.split expects STRING as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let parts = s.split(sep.as_str()).map(|p| Object::StringType(p.to_string())).collect();
    Object::Array(parts)
}

fn join(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "strings.join takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let arr = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("strings.join expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let sep = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.join expects STRING as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let parts: Vec<String> = arr.iter().map(|e| format!("{}", e)).collect();
    Object::StringType(parts.join(&sep))
}

fn contains(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "strings.contains takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let s = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.contains expects STRING as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let substr = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.contains expects STRING as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    Object::Bool(s.contains(substr.as_str()))
}

fn replace(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 3 {
        return Object::Error { message: "strings.replace takes 3 arguments: string, old, new".to_string(), line: info.line, column: info.column };
    }
    let s = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.replace expects STRING as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let old = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.replace expects STRING as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let new = match &args[2] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.replace expects STRING as third argument, got {}", args[2].type_name()), line: info.line, column: info.column },
    };
    Object::StringType(s.replace(old.as_str(), &new))
}

fn trim(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.trim takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => Object::StringType(s.trim().to_string()),
        _ => Object::Error { message: format!("strings.trim expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn trim_left(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.trim_left takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => Object::StringType(s.trim_start().to_string()),
        _ => Object::Error { message: format!("strings.trim_left expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn trim_right(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.trim_right takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => Object::StringType(s.trim_end().to_string()),
        _ => Object::Error { message: format!("strings.trim_right expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn starts_with(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "strings.starts_with takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let s = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.starts_with expects STRING as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let prefix = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.starts_with expects STRING as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    Object::Bool(s.starts_with(prefix.as_str()))
}

fn ends_with(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "strings.ends_with takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let s = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.ends_with expects STRING as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let suffix = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.ends_with expects STRING as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    Object::Bool(s.ends_with(suffix.as_str()))
}

fn index(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "strings.index takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let s = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.index expects STRING as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let substr = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.index expects STRING as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    match s.find(substr.as_str()) {
        Some(i) => Object::Integer(i as i64),
        None => Object::Integer(-1),
    }
}

fn count(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "strings.count takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let s = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.count expects STRING as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let substr = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.count expects STRING as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    Object::Integer(s.matches(substr.as_str()).count() as i64)
}

fn repeat(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "strings.repeat takes 2 arguments: string and count".to_string(), line: info.line, column: info.column };
    }
    let s = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.repeat expects STRING as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let n = match &args[1] {
        Object::Integer(n) => *n,
        _ => return Object::Error { message: format!("strings.repeat expects INTEGER as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    if n < 0 {
        return Object::Error { message: "strings.repeat: count must be non-negative".to_string(), line: info.line, column: info.column };
    }
    Object::StringType(s.repeat(n as usize))
}

fn reverse(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.reverse takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => Object::StringType(s.chars().rev().collect()),
        _ => Object::Error { message: format!("strings.reverse expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn to_chars(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.to_chars takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => Object::Array(s.chars().map(Object::Char).collect()),
        _ => Object::Error { message: format!("strings.to_chars expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn from_chars(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.from_chars takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => {
            let mut s = String::new();
            for e in elems {
                match e {
                    Object::Char(c) => s.push(*c),
                    Object::StringType(st) => s.push_str(st),
                    _ => return Object::Error { message: format!("strings.from_chars: expected CHAR elements, got {}", e.type_name()), line: info.line, column: info.column },
                }
            }
            Object::StringType(s)
        }
        _ => Object::Error { message: format!("strings.from_chars expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn parse_int(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.parse_int takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => match s.trim().parse::<i64>() {
            Ok(n) => Object::Integer(n),
            Err(_) => Object::Error { message: format!("strings.parse_int: cannot parse \"{}\" as integer", s), line: info.line, column: info.column },
        },
        _ => Object::Error { message: format!("strings.parse_int expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn parse_float(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.parse_float takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => match s.trim().parse::<f64>() {
            Ok(n) => Object::Float(n),
            Err(_) => Object::Error { message: format!("strings.parse_float: cannot parse \"{}\" as float", s), line: info.line, column: info.column },
        },
        _ => Object::Error { message: format!("strings.parse_float expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("to_upper".to_string(),    Object::Builtin(to_upper));
    members.insert("to_lower".to_string(),    Object::Builtin(to_lower));
    members.insert("split".to_string(),       Object::Builtin(split));
    members.insert("join".to_string(),        Object::Builtin(join));
    members.insert("contains".to_string(),    Object::Builtin(contains));
    members.insert("replace".to_string(),     Object::Builtin(replace));
    members.insert("trim".to_string(),        Object::Builtin(trim));
    members.insert("trim_left".to_string(),   Object::Builtin(trim_left));
    members.insert("trim_right".to_string(),  Object::Builtin(trim_right));
    members.insert("starts_with".to_string(), Object::Builtin(starts_with));
    members.insert("ends_with".to_string(),   Object::Builtin(ends_with));
    members.insert("index".to_string(),       Object::Builtin(index));
    members.insert("count".to_string(),       Object::Builtin(count));
    members.insert("repeat".to_string(),      Object::Builtin(repeat));
    members.insert("reverse".to_string(),     Object::Builtin(reverse));
    members.insert("to_chars".to_string(),    Object::Builtin(to_chars));
    members.insert("from_chars".to_string(),  Object::Builtin(from_chars));
    members.insert("parse_int".to_string(),   Object::Builtin(parse_int));
    members.insert("parse_float".to_string(), Object::Builtin(parse_float));
    members.insert("lines".to_string(),       Object::Builtin(lines));
    members.insert("is_empty".to_string(),    Object::Builtin(is_empty));
    members.insert("pad_left".to_string(),    Object::Builtin(pad_left));
    members.insert("pad_right".to_string(),   Object::Builtin(pad_right));
    Object::Module { members }
}

fn lines(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.lines takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => {
            let parts: Vec<Object> = s.lines().map(|l| Object::StringType(l.to_string())).collect();
            Object::Array(parts)
        }
        _ => Object::Error { message: format!("strings.lines expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn is_empty(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "strings.is_empty takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => Object::Bool(s.is_empty()),
        _ => Object::Error { message: format!("strings.is_empty expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn pad_left(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 3 {
        return Object::Error { message: "strings.pad_left takes 3 arguments".to_string(), line: info.line, column: info.column };
    }
    let s = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.pad_left expects STRING as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let width = match &args[1] {
        Object::Integer(n) => *n as usize,
        _ => return Object::Error { message: format!("strings.pad_left expects INTEGER as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let pad_char = match &args[2] {
        Object::StringType(c) => c.chars().next().unwrap_or(' '),
        Object::Char(c) => *c,
        _ => return Object::Error { message: format!("strings.pad_left expects STRING or CHAR as third argument, got {}", args[2].type_name()), line: info.line, column: info.column },
    };
    let char_count = s.chars().count();
    if char_count >= width {
        return Object::StringType(s);
    }
    let padding: String = std::iter::repeat(pad_char).take(width - char_count).collect();
    Object::StringType(format!("{}{}", padding, s))
}

fn pad_right(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 3 {
        return Object::Error { message: "strings.pad_right takes 3 arguments".to_string(), line: info.line, column: info.column };
    }
    let s = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("strings.pad_right expects STRING as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let width = match &args[1] {
        Object::Integer(n) => *n as usize,
        _ => return Object::Error { message: format!("strings.pad_right expects INTEGER as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let pad_char = match &args[2] {
        Object::StringType(c) => c.chars().next().unwrap_or(' '),
        Object::Char(c) => *c,
        _ => return Object::Error { message: format!("strings.pad_right expects STRING or CHAR as third argument, got {}", args[2].type_name()), line: info.line, column: info.column },
    };
    let char_count = s.chars().count();
    if char_count >= width {
        return Object::StringType(s);
    }
    let padding: String = std::iter::repeat(pad_char).take(width - char_count).collect();
    Object::StringType(format!("{}{}", s, padding))
}
