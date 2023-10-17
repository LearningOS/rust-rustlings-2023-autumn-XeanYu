// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.fn main() {}fn main() {}

#[cfg(test)]
mod tests {
    extern "C" {
        fn my_demo_function(a: u32);
        fn my_demo_function_alias(a: u32);
    }

    #[test]
    fn test_success() {
        unsafe {
            my_demo_function(123);
        }
        
        unsafe {
            my_demo_function_alias(456);
        }
    }
}