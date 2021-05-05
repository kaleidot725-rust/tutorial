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

}
