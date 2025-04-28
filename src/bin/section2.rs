pub fn main() {
    use lazy_static::lazy_static;

    println!("---- section2 ----");

    println!("数値型について");

    let mut byte: i8 = -1;
    let mut short: i16 = -1;
    let mut integer: i32 = -1;
    let mut long: i64 = -1;
    let mut too_long: i128 = -1;
    let mut float: f32 = 1.0;
    let mut double: f64 = 1.0;
    let mut boolean: bool = true;

    println!("byte: {}", byte);
    println!("short: {}", short);
    println!("integer: {}", integer);
    println!("long: {}", long);
    println!("too_long: {}", too_long);
    println!("float: {}", float);
    println!("double: {}", double);
    println!("boolean: {}", boolean);

    byte = 127;
    short = 32767;
    integer = 2147483647;
    long = 9223372036854775807;
    too_long = 170141183460469231731687303715884105727;
    float = 1.0;
    double = 1.0;
    boolean = false;

    println!("byte: {}", byte);
    println!("short: {}", short);
    println!("integer: {}", integer);
    println!("long: {}", long);
    println!("too_long: {}", too_long);
    println!("float: {}", float);
    println!("double: {}", double);
    println!("boolean: {}", boolean);

    // 数値型の変換
    let mut byte_to_short: i16 = byte as i16;
    let mut short_to_integer: i32 = short as i32;
    let mut integer_to_long: i64 = integer as i64;
    let mut long_to_too_long: i128 = long as i128;
    let mut too_long_to_float: f32 = too_long as f32;
    let mut float_to_double: f64 = float as f64;

    println!("byte_to_short: {}", byte_to_short);
    println!("short_to_integer: {}", short_to_integer);
    println!("integer_to_long: {}", integer_to_long);
    println!("long_to_too_long: {}", long_to_too_long);
    println!("too_long_to_float: {}", too_long_to_float);
    println!("float_to_double: {}", float_to_double);

    // ---------------------------------------------------

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

    // ---------------------------------------------------

    println!("タプルについて");

    let mut tuple = (1, '4', 3, "4", 5);
    println!("タプル：{}", tuple.0);
    println!("タプル：{}", tuple.1);
    println!("タプル：{}", tuple.2);
    println!("タプル：{}", tuple.3);
    println!("タプル：{}", tuple.4);

    tuple.0 = 10;
    println!("タプル：{}", tuple.0);

    // ---------------------------------------------------

    println!("ユーザ定義型について");

    println!("構造体について");
    struct User {
        name: String,
        age: u32,
    }

    println!("構造体のインスタンスを作成");
    let user = User {
        name: String::from("John"),
        age: 20,
    };
    println!("構造体のインスタンスを作成：{}", user.name);
    println!("構造体のインスタンスを作成：{}", user.age);

    println!("列挙型について");
    #[derive(Debug)]
    enum Direction {
        Up,
    }

    println!("列挙型のインスタンスを作成");
    let direction = Direction::Up;
    println!("列挙型のインスタンスを作成：{:?}", direction);

    // ---------------------------------------------------

    println!("標準ライブラリの型について");

    println!("Option型について");
    println!("データが存在する場合としない場合を表現できる列挙型");
    let mut some_number = Some(5);
    println!("Option型のインスタンスを作成：{}", some_number.unwrap());
    some_number = None;
    println!("Option型のインスタンスを作成：{}", some_number.unwrap());

    println!("Result型について");
    println!("処理結果が成功か、エラーかを表現できる列挙型");
    let mut result = Result::Ok(5);
    println!("Result型のインスタンスを作成：{}", result.unwrap());
    result = Result::Err(String::from("エラー"));
    println!("Result型のインスタンスを作成：{}", result.unwrap());

    println!("const型について");
    println!("定数を定義する型");
    const MAX_POINTS: u32 = 100000;
    println!("定数を定義：{}", MAX_POINTS);

    println!("static型について");
    println!("定数を定義する型");
    static STATIC_MAX_POINTS: u32 = 100000;
    println!("定数を定義：{}", STATIC_MAX_POINTS);

    println!("lazy_static型について");
    println!("定数を定義する型");
    lazy_static! {
        static ref LAZY_STATIC_MAX_POINTS: u32 = 100000;
    }
    println!("定数を定義：{}", *LAZY_STATIC_MAX_POINTS);

    // ---------------------------------------------------

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
        println!("所有権：所有権が移動する関数(値渡し) - 変更前 {}", s);
        // s.push_str(" mutated"); // エラー
        println!(
            "所有権：所有権が移動する関数(値渡し) - 変更後(変更できない) {}",
            s
        );
    }

    // 所有権が移動しない関数 ※参照渡し
    fn no_ownership(s: &String) {
        println!("借用：所有権が移動しない関数(参照渡し) - 変更前 {}", s);
        // s.push_str(" mutated"); // エラー
        println!(
            "借用：所有権が移動しない関数(参照渡し) - 変更後(変更できない) {}",
            s
        );
    }

    println!("借用について");

    let mut str_d = String::from("str_d");
    println!("str_d: {}", str_d);

    // 借用　所有権を移動させず不変参照を渡した例
    fn borrow_str(s: &String) {
        println!("借用：所有権を移動させず不変参照を渡した例 - 変更前 {}", s);
        // s.push_str(" mutated"); // エラー
        println!(
            "借用：所有権を移動させず不変参照を渡した例 - 変更後(変更できない) {}",
            s
        );
    }

    borrow_str(&str_d);
    println!("str_d: {}", str_d);

    // 借用　所有権を移動させず可変参照を渡した例
    fn borrow_str_mut(s: &mut String) {
        println!("借用：所有権を移動させず可変参照を渡した例 - 変更前 {}", s);
        s.push_str(" mutated");
        println!("借用：所有権を移動させず可変参照を渡した例 - 変更後 {}", s);
    }

    borrow_str_mut(&mut str_d);
    println!("str_d: {}", str_d);

    println!("ライフタイムについて");

    let str_e = String::from("str_e");
    let str_f = String::from("str_f");

    fn borrow_str_lifetime(s: &String) {
        println!("借用：ライフタイム - 変更前 {}", s);
    }

    borrow_str_lifetime(&str_e);
    borrow_str_lifetime(&str_f);

    // ライフタイムの明示的な指定
    // ライフタイムパラメータ'aを使用して、参照が有効である期間を明示
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        println!(
            "ライフタイム：明示的な指定 x, yが同じ有効期間であることを宣言, x: {}",
            x
        );
        println!(
            "ライフタイム：明示的な指定 x, yが同じ有効期間であることを宣言, y: {}",
            y
        );
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // Stringから&strへの変換
    let str_e_ref: &str = &str_e;
    let str_f_ref: &str = &str_f;

    // ライフタイムパラメータを持つ関数を呼び出し
    let longer = longest(str_e_ref, str_f_ref);
    println!("より長い文字列: {}", longer);
}
