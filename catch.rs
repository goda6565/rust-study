// エラー処理

// ?演算子
let a = get(expr)?;

// getがOption型を返すとき
// ?は以下と等しい
let a = match get(expr) {
    Some(e) => e,
    None => return Err(e),
};

// getがResult型を返すとき
// ?は以下と等しい
let a = match get(expr) {
    Ok(e) => e,
    Err(e) => return Err(e),
};

// unwrap関数
let a = get(expr).unwrap();

// getがOption型を返すとき
// unwrapは以下と等しい
let a = match get(expr) {
    Some(e) => e,
    None => { panic!() },
};

// getがResult型を返すとき
// unwrapは以下と等しい
let a = match get(expr) {
    Ok(e) => e,
    Err(e) => { panic!() },
};