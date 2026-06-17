use std::collections::HashMap;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};

use crate::object::object::{CallInfo, Object};

fn make_response(status: u16, body: String) -> Object {
    Object::Hash(vec![
        (Object::StringType("status".to_string()), Object::Integer(status as i64)),
        (Object::StringType("body".to_string()),   Object::StringType(body)),
        (Object::StringType("ok".to_string()),     Object::Bool(status >= 200 && status < 300)),
    ])
}

fn extract_headers(obj: &Object, info: &CallInfo) -> Result<HeaderMap, Object> {
    let pairs = match obj {
        Object::Hash(p) => p,
        _ => return Err(Object::Error { message: format!("headers must be HASH, got {}", obj.type_name()), line: info.line, column: info.column }),
    };
    let mut map = HeaderMap::new();
    for (k, v) in pairs {
        let key = match k {
            Object::StringType(s) => s.as_str().parse::<HeaderName>()
                .map_err(|e| Object::Error { message: format!("invalid header name: {}", e), line: info.line, column: info.column })?,
            _ => return Err(Object::Error { message: "header keys must be STRING".to_string(), line: info.line, column: info.column }),
        };
        let val = match v {
            Object::StringType(s) => HeaderValue::from_str(s)
                .map_err(|e| Object::Error { message: format!("invalid header value: {}", e), line: info.line, column: info.column })?,
            _ => return Err(Object::Error { message: "header values must be STRING".to_string(), line: info.line, column: info.column }),
        };
        map.insert(key, val);
    }
    Ok(map)
}

fn get(args: Vec<Object>, info: CallInfo) -> Object {
    if args.is_empty() || args.len() > 2 {
        return Object::Error { message: "http.get takes 1 or 2 arguments: url, [headers]".to_string(), line: info.line, column: info.column };
    }
    let url = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("http.get expects STRING url, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let client = Client::new();
    let mut req = client.get(&url);
    if args.len() == 2 {
        match extract_headers(&args[1], &info) {
            Ok(h) => req = req.headers(h),
            Err(e) => return e,
        }
    }
    match req.send() {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let body = resp.text().unwrap_or_default();
            make_response(status, body)
        }
        Err(e) => Object::Error { message: format!("http.get: {}", e), line: info.line, column: info.column },
    }
}

fn post(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() < 2 || args.len() > 3 {
        return Object::Error { message: "http.post takes 2 or 3 arguments: url, body, [headers]".to_string(), line: info.line, column: info.column };
    }
    let url = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("http.post expects STRING url, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let body = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("http.post expects STRING body, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let client = Client::new();
    let mut req = client.post(&url).body(body);
    if args.len() == 3 {
        match extract_headers(&args[2], &info) {
            Ok(h) => req = req.headers(h),
            Err(e) => return e,
        }
    }
    match req.send() {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let body = resp.text().unwrap_or_default();
            make_response(status, body)
        }
        Err(e) => Object::Error { message: format!("http.post: {}", e), line: info.line, column: info.column },
    }
}

fn post_json(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() < 2 || args.len() > 3 {
        return Object::Error { message: "http.post_json takes 2 or 3 arguments: url, body_hash, [headers]".to_string(), line: info.line, column: info.column };
    }
    let url = match &args[0] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("http.post_json expects STRING url, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let json_str = match object_to_json_string(&args[1]) {
        Ok(s) => s,
        Err(e) => return Object::Error { message: format!("http.post_json: {}", e), line: info.line, column: info.column },
    };
    let client = Client::new();
    let mut req = client.post(&url)
        .header(CONTENT_TYPE, "application/json")
        .body(json_str);
    if args.len() == 3 {
        match extract_headers(&args[2], &info) {
            Ok(h) => req = req.headers(h),
            Err(e) => return e,
        }
    }
    match req.send() {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let body = resp.text().unwrap_or_default();
            make_response(status, body)
        }
        Err(e) => Object::Error { message: format!("http.post_json: {}", e), line: info.line, column: info.column },
    }
}

fn object_to_json_string(obj: &Object) -> Result<String, String> {
    let val = object_to_serde(obj)?;
    serde_json::to_string(&val).map_err(|e| e.to_string())
}

fn object_to_serde(obj: &Object) -> Result<serde_json::Value, String> {
    match obj {
        Object::Null => Ok(serde_json::Value::Null),
        Object::Bool(b) => Ok(serde_json::Value::Bool(*b)),
        Object::Integer(n) => Ok(serde_json::Value::Number((*n).into())),
        Object::Float(f) => serde_json::Number::from_f64(*f)
            .map(serde_json::Value::Number)
            .ok_or_else(|| format!("cannot serialize float {}", f)),
        Object::StringType(s) => Ok(serde_json::Value::String(s.clone())),
        Object::Char(c) => Ok(serde_json::Value::String(c.to_string())),
        Object::Array(elems) => {
            let arr: Result<Vec<_>, _> = elems.iter().map(object_to_serde).collect();
            Ok(serde_json::Value::Array(arr?))
        }
        Object::Hash(pairs) => {
            let mut map = serde_json::Map::new();
            for (k, v) in pairs {
                let key = match k {
                    Object::StringType(s) => s.clone(),
                    _ => format!("{}", k),
                };
                map.insert(key, object_to_serde(v)?);
            }
            Ok(serde_json::Value::Object(map))
        }
        _ => Err(format!("cannot serialize {}", obj.type_name())),
    }
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("get".to_string(),       Object::Builtin(get));
    members.insert("post".to_string(),      Object::Builtin(post));
    members.insert("post_json".to_string(), Object::Builtin(post_json));
    Object::Module { members }
}
