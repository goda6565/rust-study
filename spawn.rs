use std::thread::spawn;

fn main() {

    // 基本的な使い方(クロージャでも可能)
    let _ = spawn(|| println!("Hello, world!")).join();

    // panicが起こった場合
    match spawn(|| panic!("panic")).join() {
        Ok(_) => println!("ok"),
        Err(e) => {
            let err = e.downcast_ref::<&str>(); // エラーを&str型にダウンキャスト
            println!("err: {:?}", err);
        },
    }
}