use std::io;

fn main() {
    println!("--- ユーザー入力の取得 (f64) ---");

    // String::new() で入力を受け取るための空の「器」を作る
    let mut input = String::new();

    // 1つ目の数値の取得
    // stdin().read_line() は結果を器に書き込み、成否を Result 型で返す
    // ここでは学習用なので .unwrap() でエラーを無視（失敗したらパニック）している
    io::stdin().read_line(&mut input).unwrap();

    // trim() で改行を除去し、parse::<f64>() で浮動小数点数に変換
    // ここで整数型(i32など)を指定すると、小数の入力時にクラッシュする
    let num1: f64 = input.trim().parse().unwrap();

    // 器（input）を空にしないと、2回目の入力が後ろに追記されてしまう
    input.clear();

    // 2つ目の数値の取得
    io::stdin().read_line(&mut input).unwrap();
    let num2: f64 = input.trim().parse().unwrap();

    println!("入力された値: {}, {}", num1, num2);
}
