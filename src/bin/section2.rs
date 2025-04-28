pub fn main() {
    println!("---- section2 ----");

    println!("文字列型について");

    // 可変な静的文字列
    let mut mutatable_static_str = "mutatable static str";
    println!("&str 可変な静的文字列：{}", mutatable_static_str);
    // 再代入は可能
    mutatable_static_str = "mutated mutatable static str";
    println!("&str 可変な静的文字列：{}", mutatable_static_str);
    // 文字列の追加は不可能
    // mutatable_static_str.push_str(" mutated"); // エラー
    // println!("{}", mutatable_static_str);

    // 不変な静的文字列
    let immutable_static_str = "immutable static str";
    println!("&str 不変な静的文字列：{}", immutable_static_str);
    // 再代入は不可能
    // immutable_static_str = "mutated immutable static str"; // エラー
    // println!("{}", immutable_static_str);
    // 文字列の追加は不可能
    // immutable_static_str.push_str(" mutated"); // エラー
    // println!("{}", immutable_static_str);

    // 可変な動的文字列
    let mut mutatable_dynamic_str = String::from("mutatable dynamic str");
    println!("String 可変な動的文字列：{}", mutatable_dynamic_str);
    // 再代入は可能
    mutatable_dynamic_str = String::from("mutated mutatable dynamic str");
    println!("String 可変な動的文字列：{}", mutatable_dynamic_str);
    // 文字列の追加は可能
    mutatable_dynamic_str.push_str(" mutated");
    println!("String 可変な動的文字列：{}", mutatable_dynamic_str);

    // 不変な動的文字列
    let immutable_dynamic_str = String::from("immutable dynamic str");
    println!("String 不変な動的文字列：{}", immutable_dynamic_str);
    // 再代入は不可能
    // immutable_dynamic_str = String::from("mutated immutable dynamic str"); // エラー
    // println!("{}", immutable_dynamic_str);
    // 文字列の追加は不可能
    // immutable_dynamic_str.push_str(" mutated"); // エラー
    // println!("{}", immutable_dynamic_str);
    // 文字列の追加は可能
    // immutable_dynamic_str.push_str(" mutated"); // エラー
    // println!("{}", immutable_dynamic_str);

    println!("所有権の移動について");

    // let mut str_a = String::from("str_a"); mutで変更の可能性を宣言することはできない ※所有権が移動するコード記述になっているから
    let str_a = String::from("str_a");
    let str_b = str_a;
    // println!("str_a: {}", str_a); // 所有権がないためエラー
    println!("str_b: {}", str_b);

    let str_c = String::from("str_c");
    // 所有権が移動しない関数 ※参照渡し
    no_ownership(&str_c);
    println!("str_c: {}", str_c);
    // 所有権が移動
    take_ownership(str_c);
    // 所有権が移動し、スコープを抜けているためエラー
    // println!("str_c: {}", str_c);

    // 所有権が移動する関数 ※値渡し
    fn take_ownership(s: String) {
        println!("{}", s);
    }

    // 所有権が移動しない関数 ※参照渡し
    fn no_ownership(s: &String) {
        println!("{}", s);
    }
}
