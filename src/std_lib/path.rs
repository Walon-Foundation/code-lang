use std::{collections::HashMap, path::Path};

use crate::object::object::{CallInfo, Object};

fn join(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() < 2 {
        return Object::Error { message: "path.join takes at least 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let mut result = std::path::PathBuf::new();
    for a in &args {
        match a {
            Object::StringType(s) => result.push(s),
            _ => return Object::Error { message: format!("path.join expects STRING arguments, got {}", a.type_name()), line: info.line, column: info.column },
        }
    }
    Object::StringType(result.to_string_lossy().to_string())
}

fn basename(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "path.basename takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => {
            let name = Path::new(s).file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            Object::StringType(name)
        }
        _ => Object::Error { message: format!("path.basename expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn dirname(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "path.dirname takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => {
            let parent = Path::new(s).parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| ".".to_string());
            Object::StringType(parent)
        }
        _ => Object::Error { message: format!("path.dirname expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn extension(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "path.extension takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => {
            let ext = Path::new(s).extension()
                .map(|e| e.to_string_lossy().to_string())
                .unwrap_or_default();
            Object::StringType(ext)
        }
        _ => Object::Error { message: format!("path.extension expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn stem(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "path.stem takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => {
            let stem = Path::new(s).file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            Object::StringType(stem)
        }
        _ => Object::Error { message: format!("path.stem expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn absolute(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "path.absolute takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => match std::fs::canonicalize(s) {
            Ok(p) => Object::StringType(p.to_string_lossy().to_string()),
            Err(e) => Object::Error { message: format!("path.absolute: {}", e), line: info.line, column: info.column },
        },
        _ => Object::Error { message: format!("path.absolute expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn is_absolute(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "path.is_absolute takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::StringType(s) => Object::Bool(Path::new(s).is_absolute()),
        _ => Object::Error { message: format!("path.is_absolute expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("join".to_string(),        Object::Builtin(join));
    members.insert("basename".to_string(),    Object::Builtin(basename));
    members.insert("dirname".to_string(),     Object::Builtin(dirname));
    members.insert("extension".to_string(),   Object::Builtin(extension));
    members.insert("stem".to_string(),        Object::Builtin(stem));
    members.insert("absolute".to_string(),    Object::Builtin(absolute));
    members.insert("is_absolute".to_string(), Object::Builtin(is_absolute));
    Object::Module { members }
}
