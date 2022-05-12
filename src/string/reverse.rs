pub fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(reverse("weekend"), "dnekeew");
    }

    #[test]
    fn sentence() {
        assert_eq!(reverse("write the program"), "margorp eht etirw");
    }
}
