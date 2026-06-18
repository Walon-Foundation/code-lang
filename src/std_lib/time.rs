use std::collections::HashMap;

use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};

use crate::object::object::{CallInfo, Object};

fn now(args: Vec<Object>, info: CallInfo) -> Object {
    if !args.is_empty() {
        return Object::Error { message: "time.now takes no arguments".to_string(), line: info.line, column: info.column };
    }
    Object::Integer(Utc::now().timestamp_millis())
}

fn unix(args: Vec<Object>, info: CallInfo) -> Object {
    if !args.is_empty() {
        return Object::Error { message: "time.unix takes no arguments".to_string(), line: info.line, column: info.column };
    }
    Object::Integer(Utc::now().timestamp())
}

fn sleep(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "time.sleep takes 1 argument (ms)".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Integer(ms) => {
            std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
            Object::Null
        }
        _ => Object::Error { message: format!("time.sleep expects INTEGER, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn since(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "time.since takes 1 argument (unix ms timestamp)".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Integer(start_ms) => Object::Integer(Utc::now().timestamp_millis() - start_ms),
        _ => Object::Error { message: format!("time.since expects INTEGER, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn format(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "time.format takes 2 arguments: unix_ms and layout".to_string(), line: info.line, column: info.column };
    }
    let ms = match &args[0] {
        Object::Integer(n) => *n,
        _ => return Object::Error { message: format!("time.format: first argument must be INTEGER (unix ms), got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let layout = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("time.format: second argument must be STRING layout, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let dt: DateTime<Local> = Local.timestamp_millis_opt(ms).single().unwrap_or_else(|| Local::now());
    Object::StringType(dt.format(&layout).to_string())
}

fn year(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "time.year takes 1 argument (unix ms)".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Integer(ms) => {
            let dt = Utc.timestamp_millis_opt(*ms).single().unwrap_or_else(Utc::now);
            Object::Integer(dt.year() as i64)
        }
        _ => Object::Error { message: format!("time.year expects INTEGER, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn month(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "time.month takes 1 argument (unix ms)".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Integer(ms) => {
            let dt = Utc.timestamp_millis_opt(*ms).single().unwrap_or_else(Utc::now);
            Object::Integer(dt.month() as i64)
        }
        _ => Object::Error { message: format!("time.month expects INTEGER, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn day(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "time.day takes 1 argument (unix ms)".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Integer(ms) => {
            let dt = Utc.timestamp_millis_opt(*ms).single().unwrap_or_else(Utc::now);
            Object::Integer(dt.day() as i64)
        }
        _ => Object::Error { message: format!("time.day expects INTEGER, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn hour(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "time.hour takes 1 argument (unix ms)".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Integer(ms) => {
            let dt = Utc.timestamp_millis_opt(*ms).single().unwrap_or_else(Utc::now);
            Object::Integer(dt.hour() as i64)
        }
        _ => Object::Error { message: format!("time.hour expects INTEGER, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn minute(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "time.minute takes 1 argument (unix ms)".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Integer(ms) => {
            let dt = Utc.timestamp_millis_opt(*ms).single().unwrap_or_else(Utc::now);
            Object::Integer(dt.minute() as i64)
        }
        _ => Object::Error { message: format!("time.minute expects INTEGER, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn second(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "time.second takes 1 argument (unix ms)".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Integer(ms) => {
            let dt = Utc.timestamp_millis_opt(*ms).single().unwrap_or_else(Utc::now);
            Object::Integer(dt.second() as i64)
        }
        _ => Object::Error { message: format!("time.second expects INTEGER, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("now".to_string(),    Object::Builtin(now));
    members.insert("unix".to_string(),   Object::Builtin(unix));
    members.insert("sleep".to_string(),  Object::Builtin(sleep));
    members.insert("since".to_string(),  Object::Builtin(since));
    members.insert("format".to_string(), Object::Builtin(format));
    members.insert("year".to_string(),   Object::Builtin(year));
    members.insert("month".to_string(),  Object::Builtin(month));
    members.insert("day".to_string(),    Object::Builtin(day));
    members.insert("hour".to_string(),   Object::Builtin(hour));
    members.insert("minute".to_string(), Object::Builtin(minute));
    members.insert("second".to_string(), Object::Builtin(second));
    // layout constants (chrono format strings)
    members.insert("RFC3339".to_string(),  Object::StringType("%Y-%m-%dT%H:%M:%S%z".to_string()));
    members.insert("Kitchen".to_string(),  Object::StringType("%I:%M %p".to_string()));
    Object::Module { name: "time".to_string(), pub_gated: false, members }
}
