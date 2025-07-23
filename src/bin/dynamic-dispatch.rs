use std::error::Error;
use std::fmt::{self, Display};

//　動的ディスパッチ
// 実行時に関数呼び出し先を決定する

trait Foo {
    fn foo(&self);
}

struct Bar;
impl Foo for Bar {
    fn foo(&self) {
        println!("Bar::foo");
    }
}

struct Buzz;
impl Foo for Buzz {
    fn foo(&self) {
        println!("Buzz::foo");
    }
}

// 静的ディスパッチ
fn call_foo_static<T: Foo>(arg: &T) {
    arg.foo();
}

// 動的ディスパッチ
fn call_foo_dynamic(arg: &dyn Foo) {
    arg.foo();
}

fn main() {
    let bar = Bar;
    let buzz = Buzz;

    // 静的ディスパッチ
    call_foo_static(&bar);
    call_foo_static(&buzz);

    // 動的ディスパッチ
    call_foo_dynamic(&bar);
    call_foo_dynamic(&buzz);

    let result = handle_error("ErrorA");
    println!("result: {:?}", result);
    let result = handle_error("ErrorB");
    println!("result: {:?}", result);
}

// 動的ディスパッチを利用しないといけない例
// 静的ディスパッチはコンパイル時に関数呼び出し先を決定するため、条件によって異なるエラーを返すなどはできない

#[derive(Debug)]
struct ErrorA;

#[derive(Debug)]
struct ErrorB;

impl Display for ErrorA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorA")
    }
}

impl Display for ErrorB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorB")
    }
}

impl Error for ErrorA {}
impl Error for ErrorB {}

// dyn Error はサイズ不定（unsized）な型なので、直接値として扱えない。だからヒープに格納するために Box（ポインタ）で包む必要がある。
fn handle_error(error: &str) -> Result<(), Box<dyn Error>> {
    if error == "ErrorA" {
        return Err(Box::new(ErrorA));
    } else if error == "ErrorB" {
        return Err(Box::new(ErrorB));
    }
    Ok(())
}

// | 書き方              | 意味                   |
// | ---------------- | -------------------- |
// | `&dyn Error`     | 借用参照。スタックにポインタが乗る    |
// | `Box<dyn Error>` | 所有権を持つポインタ。**ヒープ確保** |
// | `Arc<dyn Error>` | スレッド間共有するスマートポインタ    |
