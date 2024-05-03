#[test]
fn ex01() {
    trait MyTrait {
        type Output;
        fn do_something(&self) -> Self::Output;
    }

    struct MyStruct;

    impl MyTrait for MyStruct {
        type Output = i32;
        fn do_something(&self) -> Self::Output {
            42
        }
    }
    let my_struct = MyStruct {};
    assert_eq!(42, my_struct.do_something());

    struct MyStruct2;
    impl MyTrait for MyStruct2 {
        type Output = bool;
        fn do_something(&self) -> Self::Output {
            true
        }
    }
    let my_struct = MyStruct2 {};
    assert_eq!(true, my_struct.do_something());
}

#[test]
fn ex02() {
    trait MyTrait {
        fn do_something(&self) -> i32;
    }

    struct MyStruct;
    impl MyTrait for MyStruct {
        fn do_something(&self) -> i32 {
            42
        }
    }
    let my_struct = MyStruct {};
    assert_eq!(42, my_struct.do_something());
}

#[test]
fn ex03() {
    trait MyTrait {
        fn do_something(&self) -> i32;
    }
    #[allow(dead_code)]
    enum MyEnum {
        Value1,
        Value2,
        Value3,
    }
    impl MyTrait for MyEnum {
        fn do_something(&self) -> i32 {
            use MyEnum::*;
            match self {
                Value1 => 1,
                Value2 => 2,
                Value3 => 3,
            }
        }
    }
}

#[test]
fn ex04() {
    enum MyEnum {
        Value1 { x: i32, y: i32 },
        Value2 { s: String },
    }

    let x = MyEnum::Value1 { x: 1, y: 2 };
    let y = MyEnum::Value2 {
        s: "Test".to_string(),
    };
    for x in [x, y] {
        match x {
            MyEnum::Value1 { x, y } => println!("x = {}, y = {}", x, y),
            MyEnum::Value2 { s } => println!("s = {}", s),
        }
    }
}
