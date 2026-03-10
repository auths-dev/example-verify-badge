/// Minimal library crate — the point of this example is the verification badge,
/// not the Rust code.
pub fn greet() -> &'static str {
    "Hello from example-verify-badge!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello from example-verify-badge!");
    }
}
