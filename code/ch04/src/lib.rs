// Recipe#1: Declaring and Implementing Declarative Macros
//
#[test]
fn ex01() {
    macro_rules! say_hello {
        () => {
            println!("Hello, world!");
        };
    }

    say_hello!();
}
#[test]
fn ex02() {
    macro_rules! greet {
        ($name:expr) => {
            println!("Hello, {}!", $name);
        };
    }

    greet!("Anatolii");
}

#[test]
fn ex03() {
  #[macro_export]
    macro_rules! create_function {
        // This macro takes an argument of type `ident` (an identifier) and
        // creates a function named after that identifier.
        ($func_name:ident) => {
            fn $func_name() {
                // The macro will expand to a function definition, using the
                // identifier passed as an argument as the function name.
                println!("You called the function named \"{}\"", stringify!($func_name));
            }
        };
    }

    // Now we can use the macro to create a function. 
    create_function!(foo);

    // This will expand to a function definition: 
    // fn foo() { 
    //    println!("You called the function named {}", stringify!(foo)); 
    // }

    // We can now call the function we created with the macro. 
    foo();
}
