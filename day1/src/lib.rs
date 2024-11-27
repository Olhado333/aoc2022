pub fn partition(array: &mut [u32]) -> usize {
    let mut i = 0;
    let right = array.len() - 1;

    for j in 0..right {
        if array[j] < array[right] {
            array.swap(j, i);
            i += 1;
        }
    }

    array.swap(i, right);
    i
}

pub fn quicksort(array: &mut [u32]) {
    if array.len() <= 1 {
        return;
    }

    let pivot = partition(array);

    quicksort(&mut array[..pivot]);
    quicksort(&mut array[(pivot + 1)..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_result() {
        let expect = [1, 2, 3, 4, 5];
        let mut input = [4, 5, 2, 1, 3];

        quicksort(&mut input);

        assert_eq!(input, expect);
    }
}