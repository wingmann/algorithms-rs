pub fn interpolation_search(data: &[i32], target: &i32) -> Result<usize, usize> {
    if data.is_empty() {
        return Err(0);
    }
    let mut high = data.len() - 1;
    let mut low = 0usize;
    let mut interpolant = 0usize;

    loop {
        let low_value = data[low];
        let high_value = data[high];

        if high < low || *target < low_value || *target > high_value {
            break;
        }
        let offset = (*target - low_value) * (high - low) as i32 / (high_value - low_value);
        interpolant = low + offset as usize;

        let middle_value = data[interpolant];

        if middle_value > *target {
            high = interpolant - 1;
        } else if middle_value < *target {
            low = interpolant + 1;
        } else {
            break;
        }
    }

    if *target > data[high] {
        Err(high + 1)
    } else if *target < data[low] {
        Err(low)
    } else {
        Ok(interpolant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common() {
        let data = [1, 3, 7, 8, 9, 12, 22, 87];
        let search = 9;

        let result = match interpolation_search(&data, &search) {
            Ok(index) => index,
            Err(_) => 0
        };
        assert_eq!(result, 4);
    }

    #[test]
    fn not_found() {
        let data = [1, 3, 7, 8, 9, 12, 22, 87];
        let search = 2;

        let result = match interpolation_search(&data, &search) {
            Ok(index) => index,
            Err(_) => 0
        };
        assert_ne!(result, 1);
    }

    #[test]
    fn empty() {
        let data = [];
        let search = 10;

        let result = match interpolation_search(&data, &search) {
            Ok(index) => index,
            Err(error) => error
        };
        assert_eq!(result, 0);
    }
}
