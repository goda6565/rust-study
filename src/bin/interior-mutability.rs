// 内部可変性(interior mutability)
// 外側が不変な参照であっても、内部の値を変更できるということである。

// 借用
// 不変借用 &T：同時に2つ以上存在できるが、読み込みしかできない
// 可変借用 &mut T：同時に1つしか存在できないが、書き込み可能
// 不変参照と可変参照：可変参照が存在する場合、不変参照も存在できない

// この借用条件では、複数スレッドで参照可能なデータは変更できなくなる。
// その解決策が、内部可変性である。

// Cell
// Cellは共有参照を通した、変更を許す。
// 値を取り出すgetと値を置き換えるsetだけを持つ
// スレッドセーフではない

use std::cell::Cell;

// RefCell
// Cellでは、内部の値を直接参照することはできなかった、（一度getして、変更して、set）
// しかし、RefCellでは、実行コストと引き換えに、直接参照することが可能である。(borrowと、borrow_mut)
// RefCellは借用のカウンタも持つ
// Cell同様スレッドセーフではない

use std::cell::RefCell;

fn main() {
    // 借用
    let mut a = 1;
    {
        let b = &mut a;
        *b += 1;
    }
    {
        let c = &a;
        let d = &a;
        assert_eq!(c, d);
    }
    println!("{a}");

    // Cell
    let cell_a = Cell::new(10);
    let cell_b = Cell::new(20);
    switch_cell(&cell_a, &cell_b);
    println!("a = {}", cell_a.get());
    println!("b = {}", cell_b.get());

    // RefCell
    let refcell_a = RefCell::new(10);
    let refcell_b = RefCell::new(20);
    switch_refcell(&refcell_a, &refcell_b);
    println!("a = {}", *refcell_a.borrow());
    println!("b = {}", *refcell_b.borrow())
}

fn switch_cell(a: &Cell<i32>, b: &Cell<i32>) {
    let tmp = a.get();
    a.set(b.get());
    b.set(tmp);
}

fn switch_refcell(a: &RefCell<i32>, b: &RefCell<i32>) {
    let tmp = *a.borrow();
    *a.borrow_mut() = *b.borrow();
    *b.borrow_mut() = tmp;
}
