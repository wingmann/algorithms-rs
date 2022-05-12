pub fn binary_search<T>(data: &[T], target: &T) -> Result<usize, usize>
where
    T: PartialOrd,
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
    fn common() {
        assert_eq!(
            4,
            match binary_search(&[1, 3, 7, 8, 9, 12, 22, 87], &9) {
                Ok(index) => index,
                Err(_) => 0,
            }
        );
    }

    #[test]
    fn not_found() {
        assert_ne!(
            1,
            match binary_search(&[1, 3, 7, 8, 9, 12, 22, 87], &2) {
                Ok(index) => index,
                Err(_) => 0,
            }
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            0,
            match binary_search(&[], &10) {
                Ok(index) => index,
                Err(_) => 0,
            }
        );
    }
}
