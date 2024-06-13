/// By default something in this crate is a cat.
pub fn is_feline() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cats_are_cats() {
        assert_eq!(is_feline(), true);
    }
}
