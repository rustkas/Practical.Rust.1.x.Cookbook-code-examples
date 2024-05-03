#[macro_export]
macro_rules! create_function {
    // This macro takes an argument of type `ident` (an identifier) and
    // creates a function named after that identifier.
    ($func_name:ident) => {
        fn $func_name() {
            println!("func_name = {name}", name = stringify!($func_name));
            // The macro will expand to a function definition, using the
            // identifier passed as an argument as the function name.
            println!(
                "You called the function named \"{}\"",
                stringify!($func_name)
            );
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_function() {
        create_function!(foo);
        let foo: fn() = foo;
        assert_eq!(std::mem::size_of_val(&foo), std::mem::size_of::<fn()>());
        foo();
    }
}
