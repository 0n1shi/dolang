use crate::ast::{Expr, Stmt, AST};

pub fn print_ast(ast: &AST) {
    for stmt in &ast.stmts {
        print_stmt(stmt, 0);
    }
}

fn indent(level: usize) -> String {
    "  ".repeat(level)
}

fn print_stmt(stmt: &Stmt, level: usize) {
    let pad = indent(level);
    match stmt {
        Stmt::Expr(expr) => {
            println!("{pad}ExprStmt:");
            print_expr(expr, level + 1);
        }
        Stmt::Let { name, val } => {
            println!("{pad}LetStmt: {name}");
            print_expr(val, level + 1);
        }
        Stmt::Import { module } => {
            println!("{pad}ImportStmt: {module}");
        }
        Stmt::Print(expr) => {
            println!("{pad}PrintStmt:");
            print_expr(expr, level + 1);
        }
    }
}

fn print_expr(expr: &Expr, level: usize) {
    let pad = indent(level);
    match expr {
        Expr::Func { params, body } => {
            println!("{pad}Func: params = {:?}", params);
            print_expr(body, level + 1);
        }
        Expr::If { cond, then, else_ } => {
            println!("{pad}If:");
            println!("{pad}  Cond:");
            print_expr(cond, level + 2);
            println!("{pad}  Then:");
            print_expr(then, level + 2);
            println!("{pad}  Else:");
            print_expr(else_, level + 2);
        }
        Expr::Match { cond, cases } => {
            println!("{pad}Match:");
            print_expr(cond, level + 1);
            for case in cases {
                println!("{pad}  Case pattern: {:?}", case.pattern);
                print_expr(&case.body, level + 2);
            }
        }
        Expr::List(items) => {
            println!("{pad}List:");
            for item in items {
                print_expr(item, level + 1);
            }
        }
        Expr::Record(fields) => {
            println!("{pad}Record:");
            for (key, value) in fields {
                println!("{pad}  {key}:");
                print_expr(value, level + 1);
            }
        }
        Expr::Pipe { left, right } => {
            println!("{pad}Pipe:");
            print_expr(left, level + 1);
            print_expr(right, level + 1);
        }
        Expr::Logic { left, op, right } => {
            println!("{pad}Logic ({op:?}):");
            print_expr(left, level + 1);
            print_expr(right, level + 1);
        }
        Expr::Comp { left, op, right } => {
            println!("{pad}Comp ({op:?}):");
            print_expr(left, level + 1);
            print_expr(right, level + 1);
        }
        Expr::Range { start, end } => {
            println!("{pad}Range:");
            print_expr(start, level + 1);
            print_expr(end, level + 1);
        }
        Expr::Term { left, op, right } => {
            println!("{pad}Term ({op:?}):");
            print_expr(left, level + 1);
            print_expr(right, level + 1);
        }
        Expr::Factor { left, op, right } => {
            println!("{pad}Factor ({op:?}):");
            print_expr(left, level + 1);
            print_expr(right, level + 1);
        }
        Expr::Unary { op, right } => {
            println!("{pad}Unary ({op:?}):");
            print_expr(right, level + 1);
        }
        Expr::Index { list, index } => {
            println!("{pad}Index:");
            print_expr(list, level + 1);
            print_expr(index, level + 1);
        }
        Expr::Slice { list, start, end } => {
            println!("{pad}Slice:");
            print_expr(list, level + 1);
            if let Some(start) = start {
                println!("{pad}  Start:");
                print_expr(start, level + 2);
            }
            if let Some(end) = end {
                println!("{pad}  End:");
                print_expr(end, level + 2);
            }
        }
        Expr::Access { record, field } => {
            println!("{pad}Access:");
            print_expr(record, level + 1);
            println!("{pad}  Field: {field}");
        }
        Expr::Call { name, args } => {
            println!("{pad}Call:");
            print_expr(name, level + 1);
            for arg in args {
                print_expr(arg, level + 2);
            }
        }
        Expr::Identifier(name) => {
            println!("{pad}Identifier: {name}");
        }
        Expr::Number(n) => {
            println!("{pad}Number: {n}");
        }
        Expr::String(s) => {
            println!("{pad}String: \"{s}\"");
        }
        Expr::Boolean(b) => {
            println!("{pad}Boolean: {b}");
        }
    }
}
