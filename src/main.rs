mod lang;
use lang::language::Expr;
use lang::language::MutableModifier;
use lang::language::Statement;
use lang::language::Type;

use std::collections::HashMap;

fn main() {
    // let mut a = "hi";
    // let mut b = &"b";
    // let mut c = &"world";
    // if 5 == 5 {
    //     b = &a;
    // } else {
    //     c = &mut a;
    // }
    // let mut d = &mut a;
    // println!("{}, {}, {}", b, c, d);
    /* Equivalent Program:
        let mut a = "hi";
        let mut b = &"b";
        let mut c = &"world";
        if 5 == 5 {
            b = &a;
        } else {
            c = &mut a;
        }
        let mut d = &mut a;
    */
    let ifprog = Statement::Scope(vec![
        Statement::Let(
            String::from("a"),
            MutableModifier::Mutable,
            Type::String,
            Box::new(Expr::String(String::from("hi"))),
        ), // let mut a = "hi";
        Statement::Let(
            String::from("b"),
            MutableModifier::Mutable,
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference(
                MutableModifier::Immutable,
                Box::new(Expr::String(String::from("b"))),
            )),
        ), // let mut f = &"f":
        Statement::Let(
            String::from("c"),
            MutableModifier::Mutable,
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference(
                MutableModifier::Immutable,
                Box::new(Expr::String(String::from("world"))),
            )),
        ), // let mut c = &"world";
        Statement::If(
            // if false
            Box::new(Expr::Int32(0)),
            // { b = &a
            Box::new(Statement::Assign(
                String::from("b"),
                Box::new(Expr::Reference(
                    MutableModifier::Immutable,
                    Box::new(Expr::Get(String::from("a"))),
                )),
            )),
            // { c = &mut a
            Box::new(Statement::Assign(
                String::from("c"),
                Box::new(Expr::Reference(
                    MutableModifier::Mutable,
                    Box::new(Expr::Get(String::from("a"))),
                )),
            )),
        ),
        Statement::Let(
            String::from("d"),
            MutableModifier::Mutable,
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference(
                MutableModifier::Mutable,
                Box::new(Expr::Get(String::from("a"))),
            )),
        ), // let mut d = &mut a; (THIS IS THE FAILURE, HOPEFULLY :D );
    ]);
}

/**
 * borrow_check assumes the program is already type checked.
 */
fn borrow_check(program: &Statement, vars: &mut HashMap<String, MutableModifier>) -> bool {
    match program {
        Statement::Scope(vec) => {
            let mut new_vars: HashMap<String, MutableModifier> = HashMap::new();
            for (key, value) in &*vars {
                new_vars.insert(key.clone(), value.clone());
            }
            for s in vec.iter() {
                if !borrow_check(s, &mut new_vars) {
                    return false;
                }
            }
        }
        Statement::Let(_, _, _, expr) => match &**expr {
            Expr::Reference(mutable_modifier, target_name) => match &**target_name {
                Expr::Get(ref_str) => {}
                _ => (),
            },
            _ => (),
        },
        Statement::Assign(_, expr) => match &**expr {
            Expr::Reference(mutable_modifier, target_name) => {
                match vars.insert(target_name.clone(), mutable_modifier.clone()) {
                    Some(MutableModifier::Mutable) => {
                        return false;
                    }
                    // In None or Some(Not a Mutable Modifier) we just move on
                    _ => (),
                }
            }
            _ => (),
        },
        Statement::If(_, then_statement, else_statement) => {
            let mut left: HashMap<String, MutableModifier> = HashMap::new();
            let mut right: HashMap<String, MutableModifier> = HashMap::new();
            for (key, value) in &*vars {
                left.insert(key.clone(), value.clone());
                right.insert(key.clone(), value.clone());
            }
            if !(borrow_check(then_statement, &mut left)
                && borrow_check(else_statement, &mut right))
            {
                return false;
            }
            for (key, value) in left {
                match vars.insert(key.clone(), value.clone()) {
                    Some(MutableModifier::Mutable) => {
                        if matches!(MutableModifier::Immutable, value) {
                            vars.insert(key.clone(), MutableModifier::Mutable);
                        }
                    }
                    // The above insert is ok as long as we don't replace a Mutable tag with an immutable tag.
                    // If we do we want to put it back, in all other cases we keep doing inserts.
                    _ => (),
                }
            }
        }
    }
    return true;
}
