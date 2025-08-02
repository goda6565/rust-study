use std::mem;

#[allow(dead_code)]
struct Point {
    x: f64,
    y: f64,
}

enum AST {
    Number(i32),
    Add(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),
    Mul(Box<AST>, Box<AST>),
    Div(Box<AST>, Box<AST>),
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn main() {
    let point = origin();
    let box_point = Box::new(origin());
    println!(
        "point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );
    println!(
        "box_point occupies {} bytes on the stack",
        mem::size_of_val(&box_point)
    );
}
