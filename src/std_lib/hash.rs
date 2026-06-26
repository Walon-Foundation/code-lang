use std::collections::HashMap;

use crate::object::object::{CallInfo, Object};

fn obj_eq(a: &Object, b: &Object) -> bool {
    match (a, b) {
        (Object::Integer(x), Object::Integer(y)) => x == y,
        (Object::Float(x), Object::Float(y)) => x == y,
        (Object::StringType(x), Object::StringType(y)) => x == y,
        (Object::Bool(x), Object::Bool(y)) => x == y,
        (Object::Char(x), Object::Char(y)) => x == y,
        _ => false,
    }
}

fn keys(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error {
            message: "hash.keys takes 1 argument".to_string(),
            line: info.line,
            column: info.column,
        };
    }
    match &args[0] {
        Object::Hash(pairs) => Object::Array(pairs.iter().map(|(k, _)| k.clone()).collect()),
        _ => Object::Error {
            message: format!("hash.keys expects HASH, got {}", args[0].type_name()),
            line: info.line,
            column: info.column,
        },
    }
}

fn values(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error {
            message: "hash.values takes 1 argument".to_string(),
            line: info.line,
            column: info.column,
        };
    }
    match &args[0] {
        Object::Hash(pairs) => Object::Array(pairs.iter().map(|(_, v)| v.clone()).collect()),
        _ => Object::Error {
            message: format!("hash.values expects HASH, got {}", args[0].type_name()),
            line: info.line,
            column: info.column,
        },
    }
}

fn has_key(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error {
            message: "hash.has_key takes 2 arguments".to_string(),
            line: info.line,
            column: info.column,
        };
    }
    match &args[0] {
        Object::Hash(pairs) => Object::Bool(pairs.iter().any(|(k, _)| obj_eq(k, &args[1]))),
        _ => Object::Error {
            message: format!(
                "hash.has_key expects HASH as first argument, got {}",
                args[0].type_name()
            ),
            line: info.line,
            column: info.column,
        },
    }
}

fn merge(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error {
            message: "hash.merge takes 2 arguments".to_string(),
            line: info.line,
            column: info.column,
        };
    }
    let h1 = match &args[0] {
        Object::Hash(pairs) => pairs.clone(),
        _ => {
            return Object::Error {
                message: format!(
                    "hash.merge expects HASH as first argument, got {}",
                    args[0].type_name()
                ),
                line: info.line,
                column: info.column,
            };
        }
    };
    let h2 = match &args[1] {
        Object::Hash(pairs) => pairs.clone(),
        _ => {
            return Object::Error {
                message: format!(
                    "hash.merge expects HASH as second argument, got {}",
                    args[1].type_name()
                ),
                line: info.line,
                column: info.column,
            };
        }
    };
    let mut result = h1;
    for (k2, v2) in h2 {
        let mut found = false;
        for (k1, v1) in result.iter_mut() {
            if obj_eq(k1, &k2) {
                *v1 = v2.clone();
                found = true;
                break;
            }
        }
        if !found {
            result.push((k2, v2));
        }
    }
    Object::Hash(result)
}

fn delete(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error {
            message: "hash.delete takes 2 arguments".to_string(),
            line: info.line,
            column: info.column,
        };
    }
    match &args[0] {
        Object::Hash(pairs) => {
            let new: Vec<(Object, Object)> = pairs
                .iter()
                .filter(|(k, _)| !obj_eq(k, &args[1]))
                .cloned()
                .collect();
            Object::Hash(new)
        }
        _ => Object::Error {
            message: format!(
                "hash.delete expects HASH as first argument, got {}",
                args[0].type_name()
            ),
            line: info.line,
            column: info.column,
        },
    }
}

fn len(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error {
            message: "hash.len takes 1 argument".to_string(),
            line: info.line,
            column: info.column,
        };
    }
    match &args[0] {
        Object::Hash(pairs) => Object::Integer(pairs.len() as isize),
        _ => Object::Error {
            message: format!("hash.len expects HASH, got {}", args[0].type_name()),
            line: info.line,
            column: info.column,
        },
    }
}

fn entries(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error {
            message: "hash.entries takes 1 argument".to_string(),
            line: info.line,
            column: info.column,
        };
    }
    match &args[0] {
        Object::Hash(pairs) => {
            let arr = pairs
                .iter()
                .map(|(k, v)| Object::Array(vec![k.clone(), v.clone()]))
                .collect();
            Object::Array(arr)
        }
        _ => Object::Error {
            message: format!("hash.entries expects HASH, got {}", args[0].type_name()),
            line: info.line,
            column: info.column,
        },
    }
}

fn get(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 3 {
        return Object::Error {
            message: "hash.get takes 3 arguments".to_string(),
            line: info.line,
            column: info.column,
        };
    }
    match &args[0] {
        Object::Hash(pairs) => {
            for (k, v) in pairs {
                if obj_eq(k, &args[1]) {
                    return v.clone();
                }
            }
            args[2].clone()
        }
        _ => Object::Error {
            message: format!(
                "hash.get expects HASH as first argument, got {}",
                args[0].type_name()
            ),
            line: info.line,
            column: info.column,
        },
    }
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("keys".to_string(), Object::Builtin(keys));
    members.insert("values".to_string(), Object::Builtin(values));
    members.insert("has_key".to_string(), Object::Builtin(has_key));
    members.insert("merge".to_string(), Object::Builtin(merge));
    members.insert("delete".to_string(), Object::Builtin(delete));
    members.insert("len".to_string(), Object::Builtin(len));
    members.insert("entries".to_string(), Object::Builtin(entries));
    members.insert("get".to_string(), Object::Builtin(get));
    Object::Module {
        name: "hash".to_string(),
        pub_gated: false,
        members,
    }
}
