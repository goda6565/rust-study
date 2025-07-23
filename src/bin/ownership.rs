// Rustの所有権は線形論理である
// A： Apple
// B： Gold
// C： FullStomach
// A -> B: get_gold
// A -> C: eat
// A, A ⊸ B, A ⊸ C ⊢ B
// A, A ⊸ B, A ⊸ C ⊢ C
// のいずれかになる
// つまり、所有権を持つものは1つしか存在できない
// そのため、すでにお金を得ているから、満足感は得られない

#[derive(Debug)]
struct Apple {}
#[derive(Debug)]
struct Gold {}
#[derive(Debug)]
struct _FullStomach {}

// お金を得る
fn get_gold(apple: Apple) -> Gold {
    println!("get_gold: {:?}", apple);
    Gold {}
}

// 満足感を得る
fn _eat(apple: Apple) -> _FullStomach {
    println!("eat: {:?}", apple);
    _FullStomach {}
}

fn main() {
    let apple = Apple {};
    let _gold = get_gold(apple);
    // let full_stomach = eat(apple); // すでにお金を得ているから、満足感は得られない
}
