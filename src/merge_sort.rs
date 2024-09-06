/// Merge sort

fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(arr, &left, &right);
}

fn merge<T: Ord + Copy>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn merge_sort() {
        let mut arr = [8, 4, 2, 3, 5, 1, 7, 6];
        super::merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn merge() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut new_arr = vec![0; arr.len()];

        let mid = arr.len() / 2;
        let left = arr[..mid].to_vec();
        let right = arr[mid..].to_vec();

        super::merge(&mut new_arr, &left, &right);
        assert_eq!(new_arr, [1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
