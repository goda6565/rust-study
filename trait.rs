use std::ops::Add;
// trait Add<RHS = Self> {
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }

struct Vec2 {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone)] // これをつけると、コンパイラが実装を自動で生成してくれる
struct Vec2CopyClone {
    x: f64,
    y: f64,
}

impl Add for Vec2 { // Addトレイトを実装すると、+演算子を使用できるようになる
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for Vec2CopyClone {
    type Output = Vec2CopyClone;

    fn add(self, rhs: Vec2CopyClone) -> Vec2CopyClone {
        Vec2CopyClone {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


fn main() {
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { x: 3.0, y: 4.0 };
    let v3 = v1 + v2; //　所有権が移動している
    // println!("v1.x: {}", v1.x);
    // println!("v2.x: {}", v2.x);
    println!("v3.x: {}", v3.x);
    println!("v3.y: {}", v3.y);
    let v1 = Vec2CopyClone { x: 5.0, y: 6.0 };
    let v2 = Vec2CopyClone { x: 7.0, y: 8.0 };
    let _v3 = v1 + v2; // 所有権が移動していない(Copyトレイトを実装している)
    println!("v1.x: {}", v1.x);
    println!("v2.x: {}", v2.x);
    let _v4 = v1.clone() + v2.clone(); // 所有権が移動していない(Cloneトレイトを実装している)
    println!("v1.x: {}", v1.x);
    println!("v2.x: {}", v2.x);
}