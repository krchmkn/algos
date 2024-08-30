/// Quick sort
fn quick_sort(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }

    let pivot = arr[arr.len() / 2];
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        while arr[left] < pivot {
            left += 1;
        }

        while arr[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        }

        if left <= right {
            let temp = arr[left];
            arr[left] = arr[right];
            arr[right] = temp;

            left += 1;
            if right > 0 {
                right -= 1;
            }
        }

        if right > 0 {
            quick_sort(&mut arr[0..=right]);
        }
        quick_sort(&mut arr[left..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        quick_sort(&mut arr);
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn sort() {
        let mut arr = [2, 5, 4, 1, 3];
        quick_sort(&mut arr);
        assert!(arr.windows(2).all(|x| x[0] <= x[1]));
    }
}
