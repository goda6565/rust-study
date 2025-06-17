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
// つまり、すでにお金を得ているから、満足感は得られない

struct Apple {}
struct Gold {}
struct FullStomach {}

// お金を得る
fn get_gold(apple: &Apple) -> Gold {
    Gold {}
}

// 満足感を得る
fn eat(apple: &Apple) -> FullStomach {
    FullStomach {}
}

fn main() {
    let apple = Apple {};
    let gold = get_gold(apple);
    let full_stomach = eat(apple); // すでにお金を得ているから、満足感は得られない
}