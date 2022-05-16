pub mod language {
    // use std::fmt;

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
        Reference(MutableModifier, Box<Type>),
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Statement {
        Scope(Vec<Statement>),
        Let(String, MutableModifier, Type, Box<Expr>),
        Assign(String, Box<Expr>),
        If(Box<Expr>, Box<Statement>, Box<Statement>),
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Expr {
        Int32(i32),
        String(String),
        Pair(Box<Expr>, Box<Expr>),
        First(Box<Expr>),
        Second(Box<Expr>),
        Reference(String),
        Add(Box<Expr>, Box<Expr>),
        Get(String),
        Dereference(Box<Expr>),
    }
}
