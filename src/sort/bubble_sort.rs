pub fn bubble_sort(data: &mut [i32]) {
    if data.is_empty() {
        return;
    }
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 1..data.len() {
            if data[i - 1] > data[i] {
                data.swap(i - 1, i);
                swapped = true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common() {
        let mut data = [9, 1, 72, 62, 11, 98, 126, 34, 7, 5];
        let sorted_data = [1, 5, 7, 9, 11, 34, 62, 72, 98, 126];

        bubble_sort(&mut data);

        assert_eq!(data, sorted_data);
    }
}
