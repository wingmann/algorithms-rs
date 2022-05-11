use crate::search::binary_search;

pub fn exponential_search<T>(data: &[T], target: &T) -> Result<usize, usize>
    where T: PartialOrd
{
    let size = data.len();
    if size == 0 {
        return Err(0);
    }
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
    fn test_common() {
        let data = [1, 3, 7, 8, 9, 12, 22, 87];
        let search = 9;

        let result = match exponential_search(&data, &search) {
            Ok(index) => index,
            Err(_) => 0
        };
        assert_eq!(result, 4);
    }

    #[test]
    fn not_found() {
        let data = [1, 3, 7, 8, 9, 12, 22, 87];
        let search = 2;

        let result = match exponential_search(&data, &search) {
            Ok(index) => index,
            Err(_) => 0
        };
        assert_ne!(result, 1);
    }

    #[test]
    fn empty() {
        let data = [];
        let search = 10;

        let result = match exponential_search(&data, &search) {
            Ok(index) => index,
            Err(error) => error
        };
        assert_eq!(result, 0);
    }
}