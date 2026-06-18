use std::{collections::HashMap, fs, io::Write, path::Path};

use crate::object::object::{CallInfo, Object};

fn read_file(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fs.read_file takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.read_file expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    match fs::read_to_string(&path) {
        Ok(data) => Object::StringType(data),
        Err(e) => Object::Error { message: format!("fs.read_file: {}", e), line: info.line, column: info.column },
    }
}

fn write_file(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "fs.write_file takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.write_file: path must be STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let content = match &args[1] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.write_file: content must be STRING, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    match fs::write(&path, &content) {
        Ok(_) => Object::Bool(true),
        Err(e) => Object::Error { message: format!("fs.write_file: {}", e), line: info.line, column: info.column },
    }
}

fn append_file(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "fs.append_file takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.append_file: path must be STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let content = match &args[1] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.append_file: content must be STRING, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    match fs::OpenOptions::new().append(true).create(true).open(&path) {
        Ok(mut file) => match file.write_all(content.as_bytes()) {
            Ok(_) => Object::Bool(true),
            Err(e) => Object::Error { message: format!("fs.append_file: {}", e), line: info.line, column: info.column },
        },
        Err(e) => Object::Error { message: format!("fs.append_file: {}", e), line: info.line, column: info.column },
    }
}

fn read_lines(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fs.read_lines takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.read_lines expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    match fs::read_to_string(&path) {
        Ok(data) => {
            let lines = data.lines().map(|l| Object::StringType(l.to_string())).collect();
            Object::Array(lines)
        }
        Err(e) => Object::Error { message: format!("fs.read_lines: {}", e), line: info.line, column: info.column },
    }
}

fn exists(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fs.exists takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.exists expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    Object::Bool(Path::new(&path).exists())
}

fn is_file(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fs.is_file takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.is_file expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    Object::Bool(Path::new(&path).is_file())
}

fn is_dir(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fs.is_dir takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.is_dir expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    Object::Bool(Path::new(&path).is_dir())
}

fn list_dir(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fs.list_dir takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.list_dir expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    match fs::read_dir(&path) {
        Ok(entries) => {
            let mut names = Vec::new();
            for entry in entries {
                match entry {
                    Ok(e) => names.push(Object::StringType(e.file_name().to_string_lossy().to_string())),
                    Err(e) => return Object::Error { message: format!("fs.list_dir: {}", e), line: info.line, column: info.column },
                }
            }
            Object::Array(names)
        }
        Err(e) => Object::Error { message: format!("fs.list_dir: {}", e), line: info.line, column: info.column },
    }
}

fn mkdir(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fs.mkdir takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.mkdir expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    match fs::create_dir(&path) {
        Ok(_) => Object::Bool(true),
        Err(e) => Object::Error { message: format!("fs.mkdir: {}", e), line: info.line, column: info.column },
    }
}

fn mkdir_all(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fs.mkdir_all takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.mkdir_all expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    match fs::create_dir_all(&path) {
        Ok(_) => Object::Bool(true),
        Err(e) => Object::Error { message: format!("fs.mkdir_all: {}", e), line: info.line, column: info.column },
    }
}

fn remove(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fs.remove takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.remove expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    match fs::remove_file(&path) {
        Ok(_) => Object::Bool(true),
        Err(e) => Object::Error { message: format!("fs.remove: {}", e), line: info.line, column: info.column },
    }
}

fn remove_dir(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "fs.remove_dir takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    let path = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.remove_dir expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    match fs::remove_dir_all(&path) {
        Ok(_) => Object::Bool(true),
        Err(e) => Object::Error { message: format!("fs.remove_dir: {}", e), line: info.line, column: info.column },
    }
}

fn copy(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "fs.copy takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let from = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.copy: from must be STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let to = match &args[1] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.copy: to must be STRING, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    match fs::copy(&from, &to) {
        Ok(_) => Object::Bool(true),
        Err(e) => Object::Error { message: format!("fs.copy: {}", e), line: info.line, column: info.column },
    }
}

fn rename(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "fs.rename takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let from = match &args[0] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.rename: from must be STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let to = match &args[1] {
        Object::StringType(v) => v.clone(),
        _ => return Object::Error { message: format!("fs.rename: to must be STRING, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    match fs::rename(&from, &to) {
        Ok(_) => Object::Bool(true),
        Err(e) => Object::Error { message: format!("fs.rename: {}", e), line: info.line, column: info.column },
    }
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("read_file".to_string(),  Object::Builtin(read_file));
    members.insert("write_file".to_string(), Object::Builtin(write_file));
    members.insert("append_file".to_string(), Object::Builtin(append_file));
    members.insert("read_lines".to_string(), Object::Builtin(read_lines));
    members.insert("exists".to_string(),     Object::Builtin(exists));
    members.insert("is_file".to_string(),    Object::Builtin(is_file));
    members.insert("is_dir".to_string(),     Object::Builtin(is_dir));
    members.insert("list_dir".to_string(),   Object::Builtin(list_dir));
    members.insert("mkdir".to_string(),      Object::Builtin(mkdir));
    members.insert("mkdir_all".to_string(),  Object::Builtin(mkdir_all));
    members.insert("remove".to_string(),     Object::Builtin(remove));
    members.insert("remove_dir".to_string(), Object::Builtin(remove_dir));
    members.insert("copy".to_string(),       Object::Builtin(copy));
    members.insert("rename".to_string(),     Object::Builtin(rename));
    Object::Module { name: "fs".to_string(), pub_gated: false, members }
}
