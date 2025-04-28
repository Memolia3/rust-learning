# Rust の制御構文と関数

## 1. 制御構文

### if 文

Rust の`if`文は式として扱われ、値を返すことができます。

```rust
let number = 1;
if number % 4 == 0 {
    println!("4で割り切れます");
} else if number % 2 == 0 {
    println!("2で割り切れます");
} else {
    println!("その他");
}

// if式として使用
let result = if number > 0 { "正の数" } else { "負の数" };
```

特徴：

- 条件式は括弧`()`で囲む必要がない
- 条件式は必ず`bool`型を返す必要がある
- 各分岐の型は一致している必要がある

### ループ構文

#### loop 文

無限ループを実装するための構文です。

```rust
let mut count = 0;
loop {
    count += 1;
    if count == 5 {
        break;
    }
}
```

特徴：

- `break`でループを抜ける
- `continue`で次のイテレーションに進む
- 値を返すことができる（`break`の後に値を指定）

#### while 文

条件が真の間、ループを続けます。

```rust
let mut count = 0;
while count < 5 {
    println!("count: {}", count);
    count += 1;
}
```

特徴：

- 条件式は括弧`()`で囲む必要がない
- 条件式は必ず`bool`型を返す必要がある

#### for 文

イテレータを使用してループを実装します。

```rust
// Range型を使用
for i in 0..5 {
    println!("i: {}", i);
}

// 配列をイテレート
let array = [1, 2, 3, 4, 5];
for i in array {
    println!("i: {}", i);
}

// イテレータメソッドを使用
for (index, value) in array.iter().enumerate() {
    println!("index: {}, value: {}", index, value);
}
```

特徴：

- `Range`型（`0..5`）を使用可能
- コレクションを直接イテレート可能
- イテレータメソッド（`iter()`、`enumerate()`など）が利用可能

### match 文

パターンマッチングを実装するための構文です。

```rust
let number = 1;
match number {
    1 => println!("1です"),
    2 => println!("2です"),
    _ => println!("その他"),
}

// 値を返すmatch式
let result = match number {
    1 => "one",
    2 => "two",
    _ => "other",
};
```

特徴：

- 網羅的である必要がある（全ての可能性をカバー）
- パターンは上から順に評価される
- 値を返すことができる
- ガード条件（`if`）を追加可能

#### 列挙型との組み合わせ

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let direction = Direction::Up;
match direction {
    Direction::Up => println!("上"),
    Direction::Down => println!("下"),
    Direction::Left => println!("左"),
    Direction::Right => println!("右"),
}
```

特徴：

- 列挙型の全てのバリアントを網羅する必要がある
- コンパイル時に網羅性がチェックされる

## 2. 関数

### 関数の定義

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

特徴：

- 関数名はスネークケース（`snake_case`）
- 引数の型は必須
- 戻り値の型は`->`の後に指定
- 最後の式が暗黙的に返される（`return`キーワードは省略可能）

### メソッド

構造体や列挙型に実装される関数です。

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // コンストラクタ
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    // インスタンスメソッド
    fn say_hello(&self) -> &Self {
        println!("Hello, {}!", self.name);
        self
    }

    // 可変参照を取るメソッド
    fn update_age(&mut self, new_age: u32) {
        self.age = new_age;
    }
}
```

特徴：

- `impl`ブロック内で定義
- `self`パラメータでインスタンスにアクセス
- `&self`は不変参照、`&mut self`は可変参照
- メソッドチェーンが可能（`self`を返す場合）

### メソッドチェーン

```rust
let person = Person::new(String::from("John"), 20);
person.say_hello().say_age();
```

特徴：

- メソッドが`self`を返すことで連続して呼び出し可能
- コードの可読性が向上
- 中間変数が不要

### 関連関数

インスタンスを必要としない関数です。

```rust
impl Person {
    // 関連関数（staticメソッド）
    fn create_default() -> Self {
        Self {
            name: String::from("Unknown"),
            age: 0,
        }
    }
}

// 使用例
let person = Person::create_default();
```

特徴：

- `self`パラメータを持たない
- 型名::関数名で呼び出し
- コンストラクタとして使用可能

## 3. 関数の特徴

### 所有権と借用

```rust
fn take_ownership(s: String) {
    // 所有権を取得
}

fn borrow(s: &String) {
    // 不変参照を借用
}

fn borrow_mut(s: &mut String) {
    // 可変参照を借用
}
```

### ライフタイム

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### ジェネリック関数

```rust
fn print<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}
```

## 4. ベストプラクティス

### 関数の設計

- 単一の責任を持つ
- 明確な名前を付ける
- 適切なエラーハンドリングを実装
- ドキュメントコメントを追加

### メソッドの設計

- 関連する機能をグループ化
- 適切な可視性を設定
- 一貫した命名規則を使用
- メソッドチェーンを考慮

### エラーハンドリング

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```
