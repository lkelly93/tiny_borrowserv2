pub mod language {
    #[derive(Debug, Copy, Clone)]
    #[allow(dead_code)]
    pub enum MutableModifier {
        Mutable,
        Immutable,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Type {
        Int32,
        String,
        Pair(Box<Type>, Box<Type>),
        Reference(Box<Type>),
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Statement {
        Scope(Vec<Statement>),
        // Let mut a: TYPE = EXPR
        Let(String, MutableModifier, Type, Box<Expr>),
        Assign(String, Box<Expr>),
        If(Box<Expr>, Box<Statement>, Box<Statement>), // The branches of IFs must be scopes
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Expr {
        Int32(i32),
        String(String),
        Pair(Box<Expr>, Box<Expr>),
        First(Box<Expr>),
        Second(Box<Expr>),
        Reference(MutableModifier, String),
        Add(Box<Expr>, Box<Expr>),
        Get(String),
        Dereference(Box<Expr>),
    }
}
