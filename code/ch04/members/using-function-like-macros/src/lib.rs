#[macro_export]
macro_rules! add_one { // This macro takes an argument of type `expr` (an expression) and
// adds 1 to it. 
($x:expr) => { 
  $x + 1 
}; 
}

#[macro_export]
macro_rules! add { 
  ($x:expr, $y:expr) => { 
    $x + $y 
  }; 
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_one() {
      let x = add_one!(5);
      assert_eq!(6, x);
    }

    #[test]
    fn test_add() {
      let x = add!(5,5);
      assert_eq!(10, x);
    }
}
