use std::collections::HashMap;

use crate::object::object::{CallInfo, Object};

fn to_f64(obj: &Object) -> Option<f64> {
    match obj {
        Object::Integer(n) => Some(*n as f64),
        Object::Float(n)   => Some(*n),
        _ => None,
    }
}

fn float_guard(v: f64, info: CallInfo) -> Object {
    if v.is_nan() {
        Object::Error { message: "floating-point operation produced NaN".to_string(), line: info.line, column: info.column }
    } else if v.is_infinite() {
        Object::Error { message: "floating-point operation produced Infinity".to_string(), line: info.line, column: info.column }
    } else {
        Object::Float(v)
    }
}

fn sqrt(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.sqrt takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.sqrt(), info),
        None => Object::Error { message: format!("math.sqrt expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn floor(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.floor takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.floor(), info),
        None => Object::Error { message: format!("math.floor expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn ceil(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.ceil takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.ceil(), info),
        None => Object::Error { message: format!("math.ceil expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn round(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.round takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.round(), info),
        None => Object::Error { message: format!("math.round expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn trunc(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.trunc takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.trunc(), info),
        None => Object::Error { message: format!("math.trunc expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn abs(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.abs takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Integer(n) => Object::Integer(n.abs()),
        Object::Float(n)   => Object::Float(n.abs()),
        _ => Object::Error { message: format!("math.abs expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn pow(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "math.pow takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let base = match to_f64(&args[0]) {
        Some(v) => v,
        None => return Object::Error { message: format!("math.pow expects numbers, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let exp = match to_f64(&args[1]) {
        Some(v) => v,
        None => return Object::Error { message: format!("math.pow expects numbers, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    float_guard(base.powf(exp), info)
}

fn log(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.log takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.ln(), info),
        None => Object::Error { message: format!("math.log expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn log10(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.log10 takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.log10(), info),
        None => Object::Error { message: format!("math.log10 expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn exp(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.exp takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.exp(), info),
        None => Object::Error { message: format!("math.exp expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn sin(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.sin takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.sin(), info),
        None => Object::Error { message: format!("math.sin expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn cos(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.cos takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.cos(), info),
        None => Object::Error { message: format!("math.cos expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn tan(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.tan takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.tan(), info),
        None => Object::Error { message: format!("math.tan expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn min(args: Vec<Object>, info: CallInfo) -> Object {
    if args.is_empty() {
        return Object::Error { message: "math.min takes at least 1 argument".to_string(), line: info.line, column: info.column };
    }
    let mut result = match to_f64(&args[0]) {
        Some(v) => v,
        None => return Object::Error { message: format!("math.min expects numbers, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    for a in &args[1..] {
        match to_f64(a) {
            Some(v) => if v < result { result = v; },
            None => return Object::Error { message: format!("math.min expects numbers, got {}", a.type_name()), line: info.line, column: info.column },
        }
    }
    Object::Float(result)
}

fn max(args: Vec<Object>, info: CallInfo) -> Object {
    if args.is_empty() {
        return Object::Error { message: "math.max takes at least 1 argument".to_string(), line: info.line, column: info.column };
    }
    let mut result = match to_f64(&args[0]) {
        Some(v) => v,
        None => return Object::Error { message: format!("math.max expects numbers, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    for a in &args[1..] {
        match to_f64(a) {
            Some(v) => if v > result { result = v; },
            None => return Object::Error { message: format!("math.max expects numbers, got {}", a.type_name()), line: info.line, column: info.column },
        }
    }
    Object::Float(result)
}

fn clamp(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 3 {
        return Object::Error { message: "math.clamp takes 3 arguments: value, min, max".to_string(), line: info.line, column: info.column };
    }
    let v = match to_f64(&args[0]) {
        Some(v) => v,
        None => return Object::Error { message: format!("math.clamp expects numbers, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let lo = match to_f64(&args[1]) {
        Some(v) => v,
        None => return Object::Error { message: format!("math.clamp expects numbers, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let hi = match to_f64(&args[2]) {
        Some(v) => v,
        None => return Object::Error { message: format!("math.clamp expects numbers, got {}", args[2].type_name()), line: info.line, column: info.column },
    };
    Object::Float(v.clamp(lo, hi))
}

fn log2(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.log2 takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match to_f64(&args[0]) {
        Some(v) => float_guard(v.log2(), info),
        None => Object::Error { message: format!("math.log2 expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn sign(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 1 {
        return Object::Error { message: "math.sign takes 1 argument".to_string(), line: info.line, column: info.column };
    }
    match &args[0] {
        Object::Integer(n) => Object::Integer(n.signum()),
        Object::Float(f) => Object::Float(f.signum()),
        _ => Object::Error { message: format!("math.sign expects a number, got {}", args[0].type_name()), line: info.line, column: info.column },
    }
}

fn gcd(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "math.gcd takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let a = match &args[0] {
        Object::Integer(n) => n.abs(),
        _ => return Object::Error { message: format!("math.gcd expects INTEGER arguments, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let b = match &args[1] {
        Object::Integer(n) => n.abs(),
        _ => return Object::Error { message: format!("math.gcd expects INTEGER arguments, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    Object::Integer(x)
}

fn lcm(args: Vec<Object>, info: CallInfo) -> Object {
    if args.len() != 2 {
        return Object::Error { message: "math.lcm takes 2 arguments".to_string(), line: info.line, column: info.column };
    }
    let a = match &args[0] {
        Object::Integer(n) => n.abs(),
        _ => return Object::Error { message: format!("math.lcm expects INTEGER arguments, got {}", args[0].type_name()), line: info.line, column: info.column },
    };
    let b = match &args[1] {
        Object::Integer(n) => n.abs(),
        _ => return Object::Error { message: format!("math.lcm expects INTEGER arguments, got {}", args[1].type_name()), line: info.line, column: info.column },
    };
    if a == 0 || b == 0 {
        return Object::Integer(0);
    }
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    Object::Integer(a / x * b)
}

pub fn module() -> Object {
    let mut members: HashMap<String, Object> = HashMap::new();
    members.insert("PI".to_string(),    Object::Float(std::f64::consts::PI));
    members.insert("E".to_string(),     Object::Float(std::f64::consts::E));
    members.insert("sqrt".to_string(),  Object::Builtin(sqrt));
    members.insert("floor".to_string(), Object::Builtin(floor));
    members.insert("ceil".to_string(),  Object::Builtin(ceil));
    members.insert("round".to_string(), Object::Builtin(round));
    members.insert("trunc".to_string(), Object::Builtin(trunc));
    members.insert("abs".to_string(),   Object::Builtin(abs));
    members.insert("pow".to_string(),   Object::Builtin(pow));
    members.insert("log".to_string(),   Object::Builtin(log));
    members.insert("log10".to_string(), Object::Builtin(log10));
    members.insert("log2".to_string(),  Object::Builtin(log2));
    members.insert("exp".to_string(),   Object::Builtin(exp));
    members.insert("sin".to_string(),   Object::Builtin(sin));
    members.insert("cos".to_string(),   Object::Builtin(cos));
    members.insert("tan".to_string(),   Object::Builtin(tan));
    members.insert("min".to_string(),   Object::Builtin(min));
    members.insert("max".to_string(),   Object::Builtin(max));
    members.insert("clamp".to_string(), Object::Builtin(clamp));
    members.insert("sign".to_string(),  Object::Builtin(sign));
    members.insert("gcd".to_string(),   Object::Builtin(gcd));
    members.insert("lcm".to_string(),   Object::Builtin(lcm));
    Object::Module { name: "math".to_string(), pub_gated: false, members }
}
