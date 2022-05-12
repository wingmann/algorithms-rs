pub fn linear_search<T>(data: &[T], target: &T) -> Option<usize>
    where T: PartialEq
{
    data.iter().position(|x| x == target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common() {
        assert_eq!(
            4,
            match linear_search(&[1, 3, 7, 8, 9, 12, 22, 87], &9) {
                Some(index) => index,
                None => 0,
            }
        );
    }

    #[test]
    fn not_found() {
        assert_ne!(
            1,
            match linear_search(&[1, 3, 7, 8, 9, 12, 22, 87], &2) {
                Some(index) => index,
                None => 0,
            }
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            0,
            match linear_search(&[], &10) {
                Some(index) => index,
                None => 0,
            }
        );
    }
}
