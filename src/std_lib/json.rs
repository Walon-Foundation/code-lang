use std::collections::HashMap;

use serde_json::Value;

use crate::object::object::{CallInfo, Object};

fn parse(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "json.parse takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    let s = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("json.parse expects STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    match serde_json::from_str::<Value>(&s) {
        Ok(val) => json_to_object(val),
        Err(e) => Object::Error { message: format!("json.parse: {}", e), line: info.line, column: info.column },
    }
}

fn json_to_object(val: Value) -> Object {
    match val {
        Value::Null => Object::Null,
        Value::Bool(b) => Object::Bool(b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() { Object::Integer(i) }
            else { Object::Float(n.as_f64().unwrap_or(0.0)) }
        }
        Value::String(s) => Object::StringType(s),
        Value::Array(arr) => Object::Array(arr.into_iter().map(json_to_object).collect()),
        Value::Object(map) => {
            let pairs = map.into_iter()
                .map(|(k, v)| (Object::StringType(k), json_to_object(v)))
                .collect();
            Object::Hash(pairs)
        }
    }
}

fn stringify(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "json.stringify takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match object_to_json(&args[0]) {
        Ok(val) => match serde_json::to_string(&val) {
            Ok(s) => Object::StringType(s),
            Err(e) => Object::Error { message: format!("json.stringify: {}", e), line: info.line, column: info.column },
        },
        Err(msg) => Object::Error { message: format!("json.stringify: {}", msg), line: info.line, column: info.column },
    }
}

fn object_to_json(obj: &Object) -> Result<Value, String> {
    match obj {
        Object::Null => Ok(Value::Null),
        Object::Bool(b) => Ok(Value::Bool(*b)),
        Object::Integer(n) => Ok(Value::Number((*n).into())),
        Object::Float(f) => serde_json::Number::from_f64(*f)
            .map(Value::Number)
            .ok_or_else(|| format!("cannot serialize float {}", f)),
        Object::StringType(s) => Ok(Value::String(s.clone())),
        Object::Char(c) => Ok(Value::String(c.to_string())),
        Object::Array(elems) => {
            let arr: Result<Vec<Value>, _> = elems.iter().map(object_to_json).collect();
            Ok(Value::Array(arr?))
        }
        Object::Hash(pairs) => {
            let mut map = serde_json::Map::new();
            for (k, v) in pairs {
                let key = match k {
                    Object::StringType(s) => s.clone(),
                    _ => format!("{}", k),
                };
                map.insert(key, object_to_json(v)?);
            }
            Ok(Value::Object(map))
        }
        _ => Err(format!("cannot serialize {}", obj.type_name())),
    }
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("parse".to_string(),     Object::Builtin(parse));
    members.insert("stringify".to_string(), Object::Builtin(stringify));
    Object::Module { members }
}
