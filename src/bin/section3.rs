pub fn main() {
    println!("---- section3 ----");

    println!("制御構文について");

    println!("if文について");
    let number = 1;
    if number % 4 == 0 {
        println!("4で割り切れます");
    } else if number % 2 == 0 {
        println!("2で割り切れます");
    } else {
        println!("その他");
    }

    println!("loop文について");
    let mut count = 0;
    loop {
        count += 1;
        println!("count: {}", count);
        if count == 5 {
            println!("breakを設けないと一生ループする");
            break;
        }
    }

    println!("while文について");
    let mut count = 0;
    while count < 5 {
        println!("count: {}", count);
        count += 1;
    }

    println!("for文について");
    for i in 0..5 {
        println!("Range型のfor文：i: {}", i);
    }
    let array = [1, 2, 3, 4, 5];
    for i in array {
        println!("配列のfor文：i: {}", i);
    }

    println!("match文について");
    let number = 1;
    match number {
        1 => println!("1です"),
        2 => println!("2です"),
        _ => println!("その他"),
    }

    println!("列挙型のmatch文について(enum全て網羅していないとコンパイルエラーになる)");
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

    println!("関数について");

    println!("fn");
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let result = add(1, 2);
    println!("result: {}", result);

    println!("impl");

    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: String, age: u32) -> Self {
            Self { name, age }
        }

        fn say_hello(&self) -> &Self {
            println!("Hello, {}!", self.name);
            self
        }

        fn say_age(&self) -> &Self {
            println!("I am {} years old.", self.age);
            self
        }
    }

    let mut person = Person {
        name: String::from("John"),
        age: 20,
    };
    person.say_hello();
    person.say_age();

    println!("メソッドチェーン");
    let person = Person::new(String::from("John"), 20);
    person.say_hello().say_age();
}
