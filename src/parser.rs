use std::collections::HashMap;

pub struct Type(String);

pub struct Argument {
    name: String,
    data_type: Option<Type>,
}

pub struct Table {
    inner: HashMap<String, Box<Ast>>,
}

pub enum Ast {
    Variable(String),
    Call {
        fn_name: String,
        arguments: Vec<Ast>,
    },
    Block(Vec<Ast>),

    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),

    Return,

    // Types
    Table(Table),
    Number(f64),
    String(String),

    // Keywords
    Let {
        variable: String,
        data_type: Option<Type>,
        rhs: Option<Box<Ast>>,
    },

    Func {
        name: String,
        args: Vec<Argument>,
    },
    Label(String),
    Goto(String),
    If {
        condition: Box<Ast>,
        block: Box<Ast>,
        r#else: Option<Box<Ast>>,
    },
    Using {
        path: String,
        aliased_name: Option<String>,
    },
}
