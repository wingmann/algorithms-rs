use crate::search::binary_search;

pub fn exponential_search<T>(data: &[T], target: &T) -> Result<usize, usize>
    where T: PartialOrd
{
    if data.is_empty() {
        return Err(0);
    }
    let size = data.len();
    let mut high = 1usize;

    while high < size && data[high] < *target {
        high <<= 1;
    }
    let low = high >> 1;

    binary_search(&data[low..size.min(high + 1)], target)
        .map(|index| low + index)
        .map_err(|index| low + index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common() {
        assert_eq!(
            4,
            match exponential_search(&[1, 3, 7, 8, 9, 12, 22, 87], &9) {
                Ok(index) => index,
                Err(_) => 0,
            }
        );
    }

    #[test]
    fn not_found() {
        assert_ne!(
            1,
            match exponential_search(&[1, 3, 7, 8, 9, 12, 22, 87], &2) {
                Ok(index) => index,
                Err(_) => 0,
            }
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            0,
            match exponential_search(&[], &10) {
                Ok(index) => index,
                Err(_) => 0,
            }
        );
    }
}
