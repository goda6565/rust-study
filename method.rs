struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    fn norm_move(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let mut v = Vec2::new(1.0, 2.0);
    v.set(3.0, 4.0);
    println!("norm: {}", v.norm());
    let norm = v.norm_move();
    // println!("norm: {}", v.norm()); // 所有権がnormに移っているので、vは使用できない
    println!("norm: {}", norm);
}