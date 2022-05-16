mod lang;
use lang::language::Expr;
use lang::language::MutableModifier;
use lang::language::Statement;
use lang::language::Type;

use std::collections::HashMap;

fn main() {
    let mut str = "Hello World";
    str = "Goodbye World";
    println!("{}", str);
    /*
     * Equivalent Program:
     *  let str = "The Spanish Inquisition.";
     *  let str2 = &str;
     */
    let prog1 = Statement::Scope(vec![
        Statement::Let(
            String::from("str"),
            MutableModifier::Immutable,
            Type::String,
            Box::new(Expr::String(String::from("The Spanish Inquisition."))),
        ),
        Statement::Let(
            String::from("str2"),
            MutableModifier::Immutable,
            Type::String,
            Box::new(Expr::Get(String::from("str"))),
        ),
    ]);
}

fn borrow_check(program: Statement, vars: &mut HashMap<String, MutableModifier>) -> bool {
    // match program {
    //     Statement::Scope(vec) => {
    //         let mut new_vars: HashMap<String, MutableModifier> = HashMap::new();
    //         for (key, value) in &*vars {
    //             new_vars.insert(key, value)
    //         }

    //     }
    // }
    return true;
}
