use std::collections::HashMap;

use crate::object::object::{CallInfo, Evaluable, Object};

fn obj_eq(a: &Object, b: &Object) -> bool {
    match (a, b) {
        (Object::Integer(x), Object::Integer(y)) => x == y,
        (Object::Float(x),   Object::Float(y))   => x == y,
        (Object::Integer(x), Object::Float(y))   => (*x as f64) == *y,
        (Object::Float(x),   Object::Integer(y)) => *x == (*y as f64),
        (Object::StringType(x), Object::StringType(y)) => x == y,
        (Object::Bool(x),    Object::Bool(y))    => x == y,
        (Object::Char(x),    Object::Char(y))    => x == y,
        _ => false,
    }
}

// --- existing ---

fn first(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.first expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => elems.first().cloned().unwrap_or(Object::Null),
        _ => Object::Error { message: format!("arrays.first expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn last(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.last expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => elems.last().cloned().unwrap_or(Object::Null),
        _ => Object::Error { message: format!("arrays.last expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn rest(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.rest expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => {
            if elems.is_empty() { return Object::Null; }
            Object::Array(elems[1..].to_vec())
        }
        _ => Object::Error { message: format!("arrays.rest expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn push(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.push expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => {
            let mut new = elems.clone();
            new.push(args[1].clone());
            Object::Array(new)
        }
        _ => Object::Error { message: format!("arrays.push expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn len(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.len expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => Object::Integer(elems.len() as i64),
        Object::StringType(s) => Object::Integer(s.len() as i64),
        _ => Object::Error { message: format!("arrays.len expects ARRAY or STRING, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

// --- new ---

fn pop(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.pop expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => {
            if elems.is_empty() { return Object::Null; }
            Object::Array(elems[..elems.len() - 1].to_vec())
        }
        _ => Object::Error { message: format!("arrays.pop expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn prepend(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.prepend expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => {
            let mut new = vec![args[1].clone()];
            new.extend(elems.clone());
            Object::Array(new)
        }
        _ => Object::Error { message: format!("arrays.prepend expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn reverse(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.reverse expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => {
            let mut new = elems.clone();
            new.reverse();
            Object::Array(new)
        }
        _ => Object::Error { message: format!("arrays.reverse expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn contains(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.contains expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => Object::Bool(elems.iter().any(|e| obj_eq(e, &args[1]))),
        _ => Object::Error { message: format!("arrays.contains expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn index_of(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.index_of expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => {
            for (i, e) in elems.iter().enumerate() {
                if obj_eq(e, &args[1]) { return Object::Integer(i as i64); }
            }
            Object::Integer(-1)
        }
        _ => Object::Error { message: format!("arrays.index_of expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn slice(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 3 {
        return Object::Error { message: "arrays.slice expects 3 arguments".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.slice expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let start = match &args[1] {
        Object::Integer(n) => *n,
        _ => return Object::Error { message: format!("arrays.slice: start must be INTEGER, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let end = match &args[2] {
        Object::Integer(n) => *n,
        _ => return Object::Error { message: format!("arrays.slice: end must be INTEGER, got {}", args[2].type_name()), line: info.line, column: info.column },
    };
    let len = elems.len() as i64;
    let start = start.clamp(0, len) as usize;
    let end = end.clamp(0, len) as usize;
    if start >= end {
        return Object::Array(vec![]);
    }
    Object::Array(elems[start..end].to_vec())
}

fn join(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.join expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.join expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let sep = match &args[1] {
        Object::StringType(s) => s.clone(),
        _ => return Object::Error { message: format!("arrays.join: separator must be STRING, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let parts: Vec<String> = elems.iter().map(|e| format!("{}", e)).collect();
    Object::StringType(parts.join(&sep))
}

fn concat(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.concat expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let a = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.concat expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let b = match &args[1] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.concat expects ARRAY as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let mut new = a;
    new.extend(b);
    Object::Array(new)
}

fn sum(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.sum expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.sum expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let mut total_int: i64 = 0;
    let mut has_float = false;
    let mut total_float: f64 = 0.0;
    for e in &elems {
        match e {
            Object::Integer(n) => { total_int += n; total_float += *n as f64; }
            Object::Float(n)   => { has_float = true; total_float += n; }
            _ => return Object::Error { message: format!("arrays.sum: expected numeric elements, got {}", e.type_name()), line: info.line, column: info.column },
        }
    }
    if has_float { Object::Float(total_float) } else { Object::Integer(total_int) }
}

fn min(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.min expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.min expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    if elems.is_empty() { return Object::Null; }
    let mut result = elems[0].clone();
    for e in &elems[1..] {
        let less = match (&result, e) {
            (Object::Integer(a), Object::Integer(b)) => b < a,
            (Object::Float(a),   Object::Float(b))   => b < a,
            (Object::Integer(a), Object::Float(b))   => *b < (*a as f64),
            (Object::Float(a),   Object::Integer(b)) => (*b as f64) < *a,
            _ => return Object::Error { message: format!("arrays.min: expected numeric elements, got {}", e.type_name()), line: info.line, column: info.column },
        };
        if less { result = e.clone(); }
    }
    result
}

fn max(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.max expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.max expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    if elems.is_empty() { return Object::Null; }
    let mut result = elems[0].clone();
    for e in &elems[1..] {
        let greater = match (&result, e) {
            (Object::Integer(a), Object::Integer(b)) => b > a,
            (Object::Float(a),   Object::Float(b))   => b > a,
            (Object::Integer(a), Object::Float(b))   => *b > (*a as f64),
            (Object::Float(a),   Object::Integer(b)) => (*b as f64) > *a,
            _ => return Object::Error { message: format!("arrays.max: expected numeric elements, got {}", e.type_name()), line: info.line, column: info.column },
        };
        if greater { result = e.clone(); }
    }
    result
}

fn flatten(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.flatten expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => {
            let mut new = Vec::new();
            for e in elems {
                match e {
                    Object::Array(inner) => new.extend(inner.clone()),
                    other => new.push(other.clone()),
                }
            }
            Object::Array(new)
        }
        _ => Object::Error { message: format!("arrays.flatten expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn sort(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.sort expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.sort expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    if elems.is_empty() { return Object::Array(vec![]); }
    let mut new = elems.clone();
    let mut err: Option<Object> = None;
    new.sort_by(|a, b| {
        if err.is_some() { return std::cmp::Ordering::Equal; }
        match (a, b) {
            (Object::Integer(x), Object::Integer(y)) => x.cmp(y),
            (Object::Float(x),   Object::Float(y))   => x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
            (Object::Integer(x), Object::Float(y))   => (*x as f64).partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
            (Object::Float(x),   Object::Integer(y)) => x.partial_cmp(&(*y as f64)).unwrap_or(std::cmp::Ordering::Equal),
            (Object::StringType(x), Object::StringType(y)) => x.cmp(y),
            _ => {
                err = Some(Object::Error { message: format!("arrays.sort: cannot sort mixed or unsupported types"), line: info.line, column: info.column });
                std::cmp::Ordering::Equal
            }
        }
    });
    if let Some(e) = err { return e; }
    Object::Array(new)
}

fn unique(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "arrays.unique expects 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Array(elems) => {
            let mut seen: Vec<Object> = Vec::new();
            for e in elems {
                if !seen.iter().any(|s| obj_eq(s, e)) {
                    seen.push(e.clone());
                }
            }
            Object::Array(seen)
        }
        _ => Object::Error { message: format!("arrays.unique expects ARRAY, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn zip(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.zip expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let a = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.zip expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let b = match &args[1] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.zip expects ARRAY as second argument, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let pairs = a.into_iter().zip(b.into_iter())
        .map(|(x, y)| Object::Array(vec![x, y]))
        .collect();
    Object::Array(pairs)
}

fn array_map(args: Vec<Object>, info: CallInfo, eval: &mut dyn Evaluable) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.map expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.map expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let func = args[1].clone();
    let mut result = Vec::new();
    for elem in elems {
        let out = eval.call_function(func.clone(), vec![elem], info);
        if matches!(out, Object::Error { .. }) { return out; }
        result.push(out);
    }
    Object::Array(result)
}

fn array_filter(args: Vec<Object>, info: CallInfo, eval: &mut dyn Evaluable) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.filter expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.filter expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let func = args[1].clone();
    let mut result = Vec::new();
    for elem in elems {
        let out = eval.call_function(func.clone(), vec![elem.clone()], info);
        if matches!(out, Object::Error { .. }) { return out; }
        if !matches!(out, Object::Bool(false) | Object::Null) {
            result.push(elem);
        }
    }
    Object::Array(result)
}

fn array_reduce(args: Vec<Object>, info: CallInfo, eval: &mut dyn Evaluable) -> Object {
    if args.len() != 3 {
        return Object::Error { message: "arrays.reduce expects 3 arguments".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.reduce expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let func = args[1].clone();
    let mut acc = args[2].clone();
    for elem in elems {
        let out = eval.call_function(func.clone(), vec![acc, elem], info);
        if matches!(out, Object::Error { .. }) { return out; }
        acc = out;
    }
    acc
}

fn array_find(args: Vec<Object>, info: CallInfo, eval: &mut dyn Evaluable) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.find expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.find expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let func = args[1].clone();
    for elem in elems {
        let out = eval.call_function(func.clone(), vec![elem.clone()], info);
        if matches!(out, Object::Error { .. }) { return out; }
        if !matches!(out, Object::Bool(false) | Object::Null) {
            return elem;
        }
    }
    Object::Null
}

fn array_any(args: Vec<Object>, info: CallInfo, eval: &mut dyn Evaluable) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.any expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.any expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let func = args[1].clone();
    for elem in elems {
        let out = eval.call_function(func.clone(), vec![elem], info);
        if matches!(out, Object::Error { .. }) { return out; }
        if !matches!(out, Object::Bool(false) | Object::Null) {
            return Object::Bool(true);
        }
    }
    Object::Bool(false)
}

fn array_all(args: Vec<Object>, info: CallInfo, eval: &mut dyn Evaluable) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "arrays.all expects 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let elems = match &args[0] {
        Object::Array(elems) => elems.clone(),
        _ => return Object::Error { message: format!("arrays.all expects ARRAY as first argument, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let func = args[1].clone();
    for elem in elems {
        let out = eval.call_function(func.clone(), vec![elem], info);
        if matches!(out, Object::Error { .. }) { return out; }
        if matches!(out, Object::Bool(false) | Object::Null) {
            return Object::Bool(false);
        }
    }
    Object::Bool(true)
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("first".to_string(),    Object::Builtin(first));
    members.insert("last".to_string(),     Object::Builtin(last));
    members.insert("rest".to_string(),     Object::Builtin(rest));
    members.insert("push".to_string(),     Object::Builtin(push));
    members.insert("len".to_string(),      Object::Builtin(len));
    members.insert("pop".to_string(),      Object::Builtin(pop));
    members.insert("prepend".to_string(),  Object::Builtin(prepend));
    members.insert("reverse".to_string(),  Object::Builtin(reverse));
    members.insert("contains".to_string(), Object::Builtin(contains));
    members.insert("index_of".to_string(), Object::Builtin(index_of));
    members.insert("slice".to_string(),    Object::Builtin(slice));
    members.insert("join".to_string(),     Object::Builtin(join));
    members.insert("concat".to_string(),   Object::Builtin(concat));
    members.insert("sum".to_string(),      Object::Builtin(sum));
    members.insert("min".to_string(),      Object::Builtin(min));
    members.insert("max".to_string(),      Object::Builtin(max));
    members.insert("flatten".to_string(),  Object::Builtin(flatten));
    members.insert("sort".to_string(),     Object::Builtin(sort));
    members.insert("unique".to_string(),   Object::Builtin(unique));
    members.insert("zip".to_string(),      Object::Builtin(zip));
    members.insert("map".to_string(),      Object::BuiltinHigherOrder(array_map));
    members.insert("filter".to_string(),   Object::BuiltinHigherOrder(array_filter));
    members.insert("reduce".to_string(),   Object::BuiltinHigherOrder(array_reduce));
    members.insert("find".to_string(),     Object::BuiltinHigherOrder(array_find));
    members.insert("any".to_string(),      Object::BuiltinHigherOrder(array_any));
    members.insert("all".to_string(),      Object::BuiltinHigherOrder(array_all));
    Object::Module { members }
}
