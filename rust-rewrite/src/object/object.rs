use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast::ast::{Expression, Statement};

#[derive(Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    StringType(String),
    Char(char),
    Null,
    Break,
    Continue,
    Return(Box<Object>),
    Bool(bool),
    Error {
        message: String,
        line: usize,
        column: usize,
    },
    StructType {
        name: String,
        default: HashMap<String, Box<Object>>,
    },
    StructInstance {
        type_name: String,
        fields: HashMap<String, Object>,
    },
    Modulue { members: HashMap<String, Object> },
    Function {
        parameters: Vec<Expression>,
        body: Box<Statement>,
        env: Rc<RefCell<Environment>>,
    },
    Array(Vec<Object>),
    Hash(Vec<(Object, Object)>),
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[allow(unused)]
impl Object {
    pub fn type_name(&self) -> &str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Float(_) => "FLOAT",
            Object::StringType(_) => "STRING",
            Object::Char(_) => "CHAR",
            Object::Bool(_) => "BOOL",
            Object::Null => "NULL",
            Object::Break => "BREAK",
            Object::Continue => "CONTINUE",
            Object::Return(_) => "RETURN",
            Object::Error { .. } => "ERROR",
            Object::StructType { .. } => "STRUCT_TYPE",
            Object::StructInstance { .. } => "STRUCT_INSTANCE",
            Object::Modulue { .. } => "MODULE",
            Object::Function { .. } => "FUNCTION",
            Object::Array(_) => "ARRAY",
            Object::Hash(_) => "HASH",
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(v) => write!(f, "{}", v),
            Object::Float(v) => write!(f, "{}", v),
            Object::StringType(s) => write!(f, "{}", s),
            Object::Char(c) => write!(f, "{}", c),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Null => write!(f, "null"),
            Object::Break => write!(f, "break"),
            Object::Continue => write!(f, "continue"),
            Object::Return(v) => write!(f, "{}", v),
            Object::Error { message, line, column } => write!(f, "[Line {}, Column {}] ERROR: {}", line, column, message),
            Object::StructType { name, .. } => write!(f, "struct {}", name),
            Object::StructInstance { type_name, fields } => {
                let pairs: Vec<String> = fields.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{} {{ {} }}", type_name, pairs.join(", "))
            },
            Object::Modulue { .. } => write!(f, "[Module]"),
            Object::Function { parameters, .. } => {
                let params: Vec<String> = parameters.iter().map(|p| format!("{:?}", p)).collect();
                write!(f, "fn({})", params.join(", "))
            },
            Object::Array(elems) => {
                let s: Vec<String> = elems.iter().map(|e| format!("{}", e)).collect();
                write!(f, "[{}]", s.join(", "))
            },
            Object::Hash(pairs) => {
                let s: Vec<String> = pairs.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                write!(f, "{{{}}}", s.join(", "))
            },
        }
    }
}


// environment section
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub consts: HashMap<String, bool>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            consts: HashMap::new(),
            outer: None,
        }))
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            consts: HashMap::new(),
            outer: Some(outer),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }

    pub fn update(&mut self, name: &str, value: Object) -> bool {
        if self.store.contains_key(name) {
            self.store.insert(name.to_string(), value);
            return true;
        }
        match &self.outer {
            Some(outer) => outer.borrow_mut().update(name, value),
            None => false,
        }
    }

    pub fn set_const(&mut self, name: String, value: Object) {
        self.consts.insert(name.clone(), true);
        self.store.insert(name, value);
    }

    pub fn is_const(&self, name: &str) -> bool {
        if self.consts.contains_key(name) {
            true
        } else {
            match &self.outer {
                Some(outer) => outer.borrow().is_const(name),
                None => false,
            }
        }
    }
}
