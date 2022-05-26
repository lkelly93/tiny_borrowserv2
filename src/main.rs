mod lang;
use lang::language::Expr;
use lang::language::MutableModifier;
use lang::language::Statement;
use lang::language::Type;

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ReferenceInfo {
    reference_holder: String,
    reference_type: MutableModifier,
}

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
        let mut a = "a";
        let mut b = "b";
        let mut c = "c";
        if 0 {
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
            Box::new(Expr::String(String::from("a"))),
        ), // let mut a = "hi";
        Statement::Let(
            String::from("b"),
            MutableModifier::Mutable,
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::String(String::from("b"))),
        ), // let mut f = &"f":
        Statement::Let(
            String::from("c"),
            MutableModifier::Mutable,
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::String(String::from("c"))),
        ), // let mut c = &"world";
        Statement::If(
            // if false
            Box::new(Expr::Int32(0)),
            // { b = &a
            Box::new(Statement::Assign(
                String::from("b"),
                Box::new(Expr::Reference(
                    MutableModifier::Immutable,
                    String::from("a"),
                )),
            )),
            // { c = &mut a
            Box::new(Statement::Assign(
                String::from("c"),
                Box::new(Expr::Reference(MutableModifier::Mutable, String::from("a"))),
            )),
        ),
        Statement::Let(
            String::from("d"),
            MutableModifier::Mutable,
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference(MutableModifier::Mutable, String::from("a"))),
        ), // let mut d = &mut a; (THIS IS THE FAILURE, HOPEFULLY :D );
    ]);
    let mut vars: HashMap<String, HashMap<String, MutableModifier>> = HashMap::new();
    borrow_check(&ifprog, &mut vars);
    println!("Debug Point");
}
/**
 * borrow_check assumes the program is already type checked.
 */
fn borrow_check(
    program: &Statement,
    vars: &mut HashMap<String, HashMap<String, MutableModifier>>,
) -> bool {
    match program {
        // Statement::Scope(vec) => {
        //     let mut new_vars: HashMap<String, Box<Vec<ReferenceInfo>>> = HashMap::new();
        //     for (key, value) in &*vars {
        //         new_vars.insert(key.clone(), Box::new((**value).clone()));
        //     }

        //     let mut new_ass: HashMap<String, String> = HashMap::new();
        //     for (key, value) in &*assignments {
        //         new_ass.insert(key.clone(), value.clone());
        //     }
        //     for s in vec.iter() {
        //         if !borrow_check(s, &mut new_vars, &mut new_ass) {
        //             return false;
        //         }
        //     }
        //     println!("{:?}", new_vars);
        //     println!("{:?}", new_ass);
        //     return true;
        // }
        Statement::Let(new_var, _, _, expr) => vars.get(),
        Statement::Assign(old_var, expr) => {
            match assignments.get(old_var) {
                Some(s) => match vars.get_mut(s) {
                    Some(r_infos) => {
                        let mut idx = 0 as usize;
                        while idx < r_infos.len() {
                            let r_info = &r_infos[idx];
                            if old_var.eq(&r_info.reference_holder) {
                                r_infos.remove(idx);
                                continue;
                            }
                            idx += 1;
                        }
                    }
                    None => (),
                },
                None => (),
            }
            borrow_check_expr(old_var.clone(), expr, vars);
            return true;
        } // asdfas
          // Statement::If(_, then_statement, else_statement) => {
          //     let mut else_vars: HashMap<String, Box<Vec<ReferenceInfo>>> = HashMap::new();
          //     for (key, value) in &*vars {
          //         else_vars.insert(key.clone(), value.clone());
          //     }

          //     let mut else_ass: HashMap<String, String> = HashMap::new();
          //     for (key, value) in &*assignments {
          //         else_ass.insert(key.clone(), value.clone());
          //     }

          //     // Use current data for left pass. We want to merge later anyway.
          //     // No need to copy extra data.
          //     if !(borrow_check(then_statement, vars, assignments)
          //         && borrow_check(else_statement, &mut else_vars, &mut else_ass))
          //     {
          //         return false;
          //     }

          //     // Merge Everything
          //     for (key, value) in else_vars.iter_mut() {
          //         match vars.get_mut(key) {
          //             Some(old_info) => for r in old_info.iter() {},
          //             None => {
          //                 let temp = &**value;
          //                 let new_val = temp.clone();
          //                 vars.insert(key.clone(), Box::new(new_val));
          //             }
          //         }
          //     }
          //     return true;
          // }
    }
}

fn borrow_check_expr(
    new_var: String,
    expr: &Expr,
    vars: &mut HashMap<String, HashMap<String, MutableModifier>>,
) -> (bool, bool, String) {
    let mut valid = true;
    let mut is_reference = false;
    let mut reference_id: String = String::from("JUNK");
    match expr {
        Expr::Int32(_) => (),
        Expr::String(_) => (),
        Expr::Pair(a, b) => {
            // Some issues here if Reference takes an Expr::Reference instead of a string
            let (a_valid, _, _) = borrow_check_expr(new_var.clone(), a, vars);
            let (b_valid, _, _) = borrow_check_expr(new_var.clone(), b, vars);
            valid = a_valid && b_valid;
        }
        Expr::First(f) => {
            // Some issues here if Reference takes an Expr::Reference instead of a string
            let (v, _, _) = borrow_check_expr(new_var, f, vars);
            valid = v;
        }
        Expr::Second(s) => {
            // Some issues here if Reference takes an Expr::Reference instead of a string
            let (v, _, _) = borrow_check_expr(new_var, s, vars);
            valid = v;
        }
        Expr::Reference(modifier, var) => match vars.get(var) {
            Some(cur_map) => match cur_map.get(new_var)

            }
            None => {
                let new_map: HashMap<String, MutableModifier> = HashMap::new();
                new_map.insert(new_var, modifier.clone());
                vars.insert(var.clone(), new_map);
            }
        },
        Expr::Add(l, r) => {
            let (a_valid, _, _) = borrow_check_expr(new_var.clone(), l, vars);
            let (b_valid, _, _) = borrow_check_expr(new_var.clone(), r, vars);
            valid = a_valid && b_valid;
        }
        Expr::Get(_) => (),
        Expr::Dereference(_) => (),
    }
    return (valid, is_reference, reference_id);
}
