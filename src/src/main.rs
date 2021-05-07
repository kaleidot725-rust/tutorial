fn main() {
    // ハローワールド
    println!("NO1 ---------");
    {
        println!("Hello, world!");
    }
    println!("-------------");

    // 変数
    println!("NO2 ---------");
    {
        let x = 13;
        println!("{}", x);

        let x: f64 = 3.14159;
        println!("{}", x);

        let x;
        x = 0;
        println!("{}", x);

        let mut x = 42;
        println!("{}", x);
        x = 13;
        println!("{}", x);
    }
    println!("-------------");

    // 基本的な型
    println!("NO3 ---------");
    {
        let x = 12;
        let a = 12u8;
        let b = 4.3;
        let c = 4.3f32;
        let bv = true;
        let t = (13, false);
        let sentence = "hello world!";
        println!(
            "{} {} {} {} {} {} {} {}",
            x, a, b, c, bv, t.0, t.1, sentence
        );
    }
    println!("-------------");

    // 基本型の変換
    println!("NO4 ---------");
    {
        let a = 13u8;
        let b = 7u32;
        let c = a as u32 + b;
        println!("{}", c);

        let t = true;
        println!("{}", t as u8);
    }
    println!("-------------");

    // 定数
    println!("NO5 ---------");
    {
        const PI: f32 = 3.14159;
        println!("円周率 {}", PI);
        println!("-------------");
    }
    println!("-------------");

    // 配列
    println!("NO6 ---------");
    {
        let nums: [i32; 3] = [1, 2, 3];
        println!("{:?}", nums);
        println!("{}", nums[1]);
    }
    println!("-------------");

    // 配列
    println!("NO7 ---------");
    {
        fn add(x: i32, y: i32) -> i32 {
            return x + y;
        }

        println!("{}", add(42, 13));

        fn swap(x: i32, y: i32) -> (i32, i32) {
            return (y, x);
        }

        let result = swap(123, 321);
        println!("{} {}", result.0, result.1);

        let (a, b) = swap(result.0, result.1);
        println!("{} {}", a, b);

        fn make_nothing() -> () {
            return ();
        }
        let a = make_nothing();
        println!("a の値: {:?}", a);
    }
    println!("-------------");

    // 構造体
    println!("No8 ----------");
    {
        struct SeaCreature {
            animal_type: String,
            name: String,
            arms: i32,
            legs: i32,
            weapon: String
        }

        // スタティックメソッド
        let s = String::from("Hello world!");

        // インスタンスメソッド
        println!("{} is {} characters long.", s, s.len());
    }
    println!("-------------");
}
