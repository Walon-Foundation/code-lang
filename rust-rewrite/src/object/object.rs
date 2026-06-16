use std::{cell::RefCell, collections::HashMap, rc::Rc};

// object section
#[derive(Debug, Clone)]
pub enum Object {
    Integer(i64),
    Float(f64),
    StringType(String),
    Char(char),
    Null,
    Return(Box<Object>),
    Bool(bool),
    Error{
        message:String,
        line:usize,
        column:usize,
    },
}

#[allow(unused)]
impl Object {
    pub fn type_name(&self) -> &str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Null => "NULL",
            Object::Error { message,line,column } => "ERROR",
            Object::Return(_) => "RETURN",
            Object::Float(_) => "FLOAT",
            Object::StringType(_) => "STRING",
            Object::Char(_) => "CHAR",
            Object::Bool(_) => "BOOL",
            _ => "NULL"
        }
    }
}


impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(v) => write!(f, "{}", v),
            Object::StringType(s) => write!(f, "{}", s), 
            Object::Float(v) => write!(f, "{}",v),
            Object::Error { message, line, column } => write!(f, "[Line {}, Column {}] ERROR: {}", line, column, message),
            Object::Null => write!(f, "null"),
            Object::Return(v) => write!(f, "{}", v),
            Object::Char(c) => write!(f, "{}", c),
            Object::Bool(b) => write!(f, "{}",b),
        }
    }
}


// environment section
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub consts: HashMap<String, bool>,
    outer: Option<Rc<RefCell<Environment>>>
}

impl Environment {
    pub fn new() -> Rc<RefCell<Environment>>{
        Rc::new(RefCell::new(
            Environment { store: HashMap::new(), consts: HashMap::new(), outer: None }
        ))
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(
            Environment { store: HashMap::new(), consts: HashMap::new(), outer:Some(outer) }
        ))
    }
    
    pub fn get(&self, name:&str) -> Option<Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => {
                match &self.outer {
                    Some(outer) => outer.borrow().get(name),
                    None => None
                }
            }
        }
    }
    
    pub fn set(&mut self, name:String,value:Object) {
        self.store.insert(name, value);
    }

    // const binding
    pub fn set_const(&mut self, name: String, value: Object) {
        self.consts.insert(name.clone(), true);
        self.store.insert(name, value);
    }

    // is this name const, anywhere in the chain?
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