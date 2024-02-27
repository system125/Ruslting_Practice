// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    if num == 0 {
        1
    }else if num > 0{
        let iter_end = num +1;
        let iters = (1..iter_end) .into_iter();
        iters.reduce(|prev,next| prev * next).unwrap_or_default()
    }else{
        panic!("No negative numbers allowed")
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        let fact_4 = factorial(4);
        assert_eq!(24, fact_4);
    }
}
