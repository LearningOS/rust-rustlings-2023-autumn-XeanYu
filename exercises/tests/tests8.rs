// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // Set the TEST_FOO environment variable to the desired value.
    println!("cargo:rerun-if-env-changed=TEST_FOO");
    println!("cargo:TEST_FOO=123"); // Change the value to the desired timestamp

    // Set the "pass" feature to enable the conditional compilation.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
