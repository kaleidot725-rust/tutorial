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
            weapon: String,
        }

        // スタティックメソッド
        let s = String::from("Hello world!");

        // インスタンスメソッド
        println!("{} is {} characters long.", s, s.len());

        let ferris = SeaCreature {
            animal_type: String::from("crab"),
            name: String::from("Ferris"),
            arms: 2,
            legs: 4,
            weapon: String::from("claw"),
        };

        println!(
            "{} is a {}. They have {} arms, {} legs, and a {} weapon",
            ferris.animal_type, ferris.name, ferris.arms, ferris.legs, ferris.weapon
        );

        let sarah = SeaCreature {
            animal_type: String::from("octopus"),
            name: String::from("Sarah"),
            arms: 8,
            legs: 0,
            weapon: String::from("none"),
        };

        println!(
            "{} is a {}. They have {} arms, {} legs, and a {} weapon",
            sarah.animal_type, sarah.name, sarah.arms, sarah.legs, sarah.weapon
        );

        struct Location(i32, i32);
        let loc = Location(42, 32);
        println!("{}, {}", loc.0, loc.1);

        struct Marker;
        let _m = Marker;
    }
    println!("-------------");

    // 列挙型
    println!("No9 ----------");
    {
        enum Species {
            Crab,
            Octopus,
            Fish,
            Clam,
        }

        struct SeaCreature {
            species: Species,
            name: String,
            arms: i32,
            legs: i32,
            weapon: String,
        }

        let ferris = SeaCreature {
            species: Species::Crab,
            name: String::from("Ferris"),
            arms: 2,
            legs: 4,
            weapon: String::from("claw"),
        };

        match ferris.species {
            Species::Crab => println!("{} is a crab", ferris.name),
            Species::Octopus => println!("{} is a octopus", ferris.name),
            Species::Fish => println!("{} is a fish", ferris.name),
            Species::Clam => println!("{} is a clam", ferris.name)
        }
    }
    println!("-------------");

    // 列挙型でUnion
    println!("No10 ----------");
    {
        enum Species { Crab, Octopus, Fish, Clam }
        enum PoisonType { Acidic, Painful, Lethal }
        enum Size { Big, Small }
        enum Weapon {
            Claw(i32, Size),
            Poison(PoisonType),
            None,
        }

        struct SeaCreature {
            species: Species,
            name: String,
            arms: i32,
            legs: i32,
            weapon: Weapon,
        }

        let ferris = SeaCreature {
            species: Species::Crab,
            name: String::from("Ferris"),
            arms: 2,
            legs: 4,
            weapon: Weapon::Claw(2, Size::Small),
        };

        match ferris.species {
            Species::Crab => {
                match ferris.weapon {
                    Weapon::Claw(num_claws, size) => {
                        let size_description = match size {
                            Size::Big => "big",
                            Size::Small => "small"
                        };
                        println!("ferris is a crab with {} {} claws", num_claws, size_description)
                    }
                    _ => println!("ferris is a  crab with some other weapon")
                }
            },
            _ => println!("ferris is some other animal"),
        }
    }
    println!("-------------");
}
