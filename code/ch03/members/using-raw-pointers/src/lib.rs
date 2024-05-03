// Recipe#4: Working with Raw Pointers
#[test]
fn ex01() {
    let mut x = 10;
    let y = 20;
    let p: *mut i32 = &mut x;
    unsafe {
        *p = y;
    }

    println!("x = {x}");
    assert_eq!(x, 20);
}

#[test]
fn ex02() {
    let mut x = 10;
    let mut y = 20;

    let p: *mut i32 = &mut x;
    let q: *mut i32 = &mut y;
    unsafe {
        *p = *q;
    }

    println!("x = {x}");
    assert_eq!(x, 20);
}

// Recipe#5: Working with Smart Pointers
#[test]
fn ex03() {
    let x = 10;
    let y = Box::new(20);
    println!("x = {}", x);
    println!("y = {}", *y);

    assert_eq!(10, x);
    assert_eq!(20, *y);
}

#[test]
fn ex04() {
    use std::fmt;
    use std::rc::Rc;

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Rc<Node>>,
    }
    impl fmt::Display for Node {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.value)?;

            let mut current = &self.next;
            while let Some(node) = current {
                write!(f, " -> {}", node.value)?;
                current = &node.next;
            }

            Ok(())
        }
    }

    let a = Rc::new(Node {
        value: 10,
        next: None,
    });
    let b = Rc::new(Node {
        value: 20,
        next: Some(a.clone()),
    });
    let c = Rc::new(Node {
        value: 30,
        next: Some(b.clone()),
    });
    println!("a = {a}");
    println!("b = {b}");
    println!("c = {c}");
}

// Recipe#6: Using ‘mod’ Modules

#[test]

fn ex05() {
    #[allow(dead_code)]
    mod my_module {
        fn my_function() {
            // function body goes here
        }
        #[derive(Debug)]
        pub(crate) struct MyStruct {
            // struct fields go here
        }
        impl MyStruct {
            pub(crate) fn new() -> MyStruct {
                // implementation goes here
                MyStruct {}
            }
        }
    }
    use my_module::MyStruct;
    let _s = MyStruct::new();
    // or
    let _t = my_module::MyStruct::new();
}
