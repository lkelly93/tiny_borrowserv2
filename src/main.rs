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
    //     c = &a;
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
            c = &a;
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
            Box::new(Statement::Scope(vec![Statement::Assign(
                String::from("b"),
                Box::new(Expr::Reference(
                    MutableModifier::Immutable,
                    String::from("a"),
                )),
            )])),
            Box::new(Statement::Scope(vec![Statement::Assign(
                String::from("c"),
                Box::new(Expr::Reference(
                    MutableModifier::Immutable,
                    String::from("a"),
                )),
            )])),
            // { c = &mut a
        ),
        Statement::Let(
            String::from("d"),
            MutableModifier::Mutable,
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference(MutableModifier::Mutable, String::from("a"))),
        ), // let mut d = &mut a; (THIS IS THE FAILURE, HOPEFULLY :D );
    ]);
    // borrow_check(&ifprog);

    // Same as ifprog1 but valid.
    let ifprog2 = Statement::Scope(vec![
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
            Box::new(Statement::Scope(vec![Statement::Assign(
                String::from("b"),
                Box::new(Expr::Reference(
                    MutableModifier::Immutable,
                    String::from("a"),
                )),
            )])),
            Box::new(Statement::Scope(vec![Statement::Assign(
                String::from("c"),
                Box::new(Expr::Reference(
                    MutableModifier::Immutable,
                    String::from("a"),
                )),
            )])),
            // { c = &mut a
        ),
    ]);
    // borrow_check(&ifprog2);

    /* Equivalent Program:
        let mut a = "a";
        let mut b = "b";
        let mut c = "c";
        if 0 {
            c = &a;
        } else {
            c = &b;
        }
        let d = &mut a;
    */
    // This is a test case for different branches with same lvalue.
    let ifprog3 = Statement::Scope(vec![
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
            Box::new(Statement::Scope(vec![Statement::Assign(
                String::from("c"),
                Box::new(Expr::Reference(
                    MutableModifier::Immutable,
                    String::from("a"),
                )),
            )])),
            Box::new(Statement::Scope(vec![Statement::Assign(
                String::from("c"),
                Box::new(Expr::Reference(
                    MutableModifier::Immutable,
                    String::from("b"),
                )),
            )])),
            // { c = &mut a
        ),
        /*
         * This should fail whether I mut borrow a or b.
         */
        // Statement::Let(
        //     String::from("d"),
        //     MutableModifier::Mutable,
        //     Type::Reference(Box::new(Type::String)),
        //     Box::new(Expr::Reference(MutableModifier::Mutable, String::from("a"))),
        // ), // let mut d = &mut a; (THIS IS THE FAILURE, HOPEFULLY :D );
        Statement::Let(
            String::from("d"),
            MutableModifier::Mutable,
            Type::Reference(Box::new(Type::String)),
            Box::new(Expr::Reference(MutableModifier::Mutable, String::from("b"))),
        ), // let mut d = &mut a; (THIS IS THE FAILURE, HOPEFULLY :D );
    ]);
    borrow_check(&ifprog3);
}

fn borrow_check(program: &Statement) {
    let mut referenced_map: HashMap<String, HashMap<String, MutableModifier>> = HashMap::new();
    let mut assignments_map: HashMap<String, Vec<String>> = HashMap::new();
    match program {
        Statement::Scope(statements) => {
            for s in statements.iter() {
                // println!("{:?}", s);
                if !borrow_check_helper(s, &mut referenced_map, &mut assignments_map) {
                    println!("Invalid on {:?}", s);
                    return;
                }
                for (key, value) in referenced_map.iter() {
                    println!("{}:{:?}", key, value);
                }
            }
            println!("Valid!");
        }
        _ => println!("Only call me with Statement::Scopes."),
    }
}

/**
 * borrow_check assumes the program is already type checked.
 */
fn borrow_check_helper(
    program: &Statement,
    referenced_map: &mut HashMap<String, HashMap<String, MutableModifier>>,
    assignments_map: &mut HashMap<String, Vec<String>>,
) -> bool {
    match program {
        Statement::Scope(statements) => {
            let mut new_ref_map = referenced_map.clone();
            let mut new_assign_map = assignments_map.clone();
            for statement in statements.iter() {
                if !borrow_check_helper(statement, &mut new_ref_map, &mut new_assign_map) {
                    return false;
                }
            }
            return true;
        }
        Statement::If(_, then_case, else_case) => match (&**then_case, &**else_case) {
            (Statement::Scope(then_scope), Statement::Scope(else_scope)) => {
                let mut then_ref_map = referenced_map.clone();
                let mut then_assign_map = assignments_map.clone();
                for statement in then_scope.iter() {
                    if !borrow_check_helper(statement, &mut then_ref_map, &mut then_assign_map) {
                        return false;
                    }
                }

                let mut else_ref_map = referenced_map.clone();
                let mut else_assign_map = assignments_map.clone();
                for statement in else_scope.iter() {
                    if !borrow_check_helper(statement, &mut else_ref_map, &mut else_assign_map) {
                        return false;
                    }
                }
                // Since we only allow scopes in Ifs we only care about variables that existed before the If
                for (key, value) in assignments_map.iter_mut() {
                    let then_vec = then_assign_map.get_mut(key).expect("This should never happen, Luke(this is actually always true) is an idiot or you wrote your program wrong if it does.");
                    // This could cause duplicates in the vec... I don't think we care about this.
                    value.append(&mut then_vec.clone());
                    for referenced_var in then_vec.iter() {
                        let new_mod = then_ref_map
                            .get(referenced_var)
                            .expect("it exists")
                            .get(key)
                            .expect("it also exists")
                            .clone();

                        let previous = referenced_map
                            .entry(referenced_var.clone())
                            .or_insert(HashMap::new()) // If we insert that means there has never been a reference to this var before.
                            .insert(key.clone(), new_mod);
                        if matches!(previous, Some(MutableModifier::Mutable)) {
                            if matches!(new_mod, MutableModifier::Mutable) {
                                // Double Mutable, FAIL.
                                println!("Two mutable references of {} found.", referenced_var);
                                return false;
                            } else {
                                // We overwrote a Mutable with Immutable, undo that.
                                referenced_map
                                    .get_mut(referenced_var)
                                    .expect("WE LITERALLY ALREADY SAW IT. WTF?!")
                                    .insert(key.clone(), MutableModifier::Mutable);
                            }
                        }
                    }

                    let else_vec = else_assign_map.get_mut(key).expect("This should never happen, Luke is an idiot or you wrote your program wrong if it does.");
                    value.append(&mut else_vec.clone());
                    for referenced_var in else_vec.iter() {
                        let new_mod = else_ref_map
                            .get(referenced_var)
                            .expect("it exists")
                            .get(key)
                            .expect("it also exists")
                            .clone();

                        let previous = referenced_map
                            .entry(referenced_var.clone())
                            .or_insert(HashMap::new()) // If we insert that means there has never been a reference to this var before.
                            .insert(key.clone(), new_mod);
                        if matches!(previous, Some(MutableModifier::Mutable)) {
                            if matches!(new_mod, MutableModifier::Mutable) {
                                // Double Mutable, FAIL.
                                println!("Two mutable references of {} found.", referenced_var);
                                return false;
                            } else {
                                // We overwrote a Mutable with Immutable, undo that.
                                referenced_map
                                    .get_mut(referenced_var)
                                    .expect("WE LITERALLY ALREADY SAW IT. WTF?!")
                                    .insert(key.clone(), MutableModifier::Mutable);
                            }
                        }
                    }
                }
                return true;
            }
            _ => {
                println!("If branches must be Scopes.");
                return false;
            }
        },
        Statement::Let(lvalue, _, _, rvalue) => match borrow_check_expr(rvalue) {
            None => {
                assignments_map.insert(lvalue.clone(), Vec::new());
                return true;
            }
            Some(vec) => {
                if !assignments_map.contains_key(lvalue) {
                    let tmp = Vec::new();
                    assignments_map.insert(lvalue.clone(), tmp);
                }
                let assignment_vec: &mut Vec<String> =
                    assignments_map.get_mut(lvalue).expect("Fatal Error!");

                for (modifier, referenced_var) in vec.iter() {
                    assignment_vec.push(referenced_var.clone());

                    if !referenced_map.contains_key(referenced_var) {
                        let tmp: HashMap<String, MutableModifier> = HashMap::new();
                        referenced_map.insert(referenced_var.clone(), tmp);
                    }
                    let map_of_referencees: &mut HashMap<String, MutableModifier> = referenced_map
                        .get_mut(referenced_var)
                        .expect("Fatal Error!");

                    if matches!(modifier, MutableModifier::Mutable) {
                        // No other references allowed when mutable reference exists.
                        if map_of_referencees.len() != 0 {
                            return false;
                        }
                    }
                    map_of_referencees.insert(lvalue.clone(), modifier.clone());
                }
                return true;
            }
        },
        Statement::Assign(lvalue, rvalue) => match borrow_check_expr(rvalue) {
            // Clear assignment vecl
            None => return true,
            Some(vec) => {
                if matches!(assignments_map.get(lvalue), None) {
                    // This variable has never been a reference before.
                    assignments_map.insert(lvalue.clone(), Vec::new());
                }
                let assignment_vec: &mut Vec<String> = assignments_map
                    .get_mut(lvalue)
                    .expect("This should never happen... I just inserted it.");
                for referenced_var in assignment_vec.iter() {
                    referenced_map
                        .get_mut(referenced_var)
                        .expect("referenced_var not in referencedMap this is a type error.")
                        .remove(lvalue);
                }
                // If this is a new reference... thats ok even though this clear is wasted cycles.
                assignment_vec.clear();
                for (modifier, referenced_var) in vec.iter() {
                    assignment_vec.push(referenced_var.clone());

                    if !referenced_map.contains_key(referenced_var) {
                        let tmp: HashMap<String, MutableModifier> = HashMap::new();
                        referenced_map.insert(referenced_var.clone(), tmp);
                    }
                    let map_of_referencees: &mut HashMap<String, MutableModifier> = referenced_map
                        .get_mut(referenced_var)
                        .expect("I just put it there!?!?");

                    if matches!(modifier, MutableModifier::Mutable) {
                        for (_, mod_type) in map_of_referencees.iter() {
                            if matches!(mod_type, MutableModifier::Mutable) {
                                return false;
                            }
                        }
                    }
                    map_of_referencees.insert(lvalue.clone(), modifier.clone());
                }
                return true;
            }
        },
    }
}

fn borrow_check_expr(expr: &Expr) -> Option<Vec<(MutableModifier, String)>> {
    match expr {
        Expr::Int32(_) => None,
        Expr::String(_) => None,
        Expr::Pair(a, b) => match (borrow_check_expr(a), borrow_check_expr(b)) {
            (None, None) => None,
            (r, None) => r,
            (None, r) => r,
            (Some(vec_left), Some(vec_right)) => {
                let mut comb_vec = vec_left.clone();
                for i in vec_right.iter() {
                    comb_vec.push(i.clone());
                }
                return Option::Some(comb_vec);
            }
        },
        Expr::First(f) => borrow_check_expr(f),
        Expr::Second(s) => borrow_check_expr(s),
        Expr::Reference(modifier, var) => Option::Some(vec![(*modifier, var.clone())]),
        // Correct me if I am wrong but add can't add references???
        Expr::Add(_, _) => None,
        Expr::Get(_) => None,
        Expr::Dereference(_) => None,
    }
}
