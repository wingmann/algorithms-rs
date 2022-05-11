pub fn binary_search<T>(data: &[T], target: &T) -> Result<usize, usize>
    where T: PartialOrd
{
    if data.is_empty() {
        return Err(0);
    }
    let mut size = data.len();
    let mut base = 0usize;

    while size > 1 {
        let half = size / 2;
        let middle = base + half;

        if data[middle] <= *target {
            base = middle;
        }
        size -= half;
    }

    if data[base] == *target {
        Ok(base)
    } else {
        Err(base + (data[base] < *target) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common() {
        let data = [1, 3, 7, 8, 9, 12, 22, 87];
        let search = 9;

        let result = match binary_search(&data, &search) {
            Ok(index) => index,
            Err(_) => 0
        };
        assert_eq!(result, 4);
    }

    #[test]
    fn not_found() {
        let data = [1, 3, 7, 8, 9, 12, 22, 87];
        let search = 2;

        let result = match binary_search(&data, &search) {
            Ok(index) => index,
            Err(_) => 0
        };
        assert_ne!(result, 1);
    }

    #[test]
    fn empty() {
        let data = [];
        let search = 10;

        let result = match binary_search(&data, &search) {
            Ok(index) => index,
            Err(error) => error
        };
        assert_eq!(result, 0);
    }
}
