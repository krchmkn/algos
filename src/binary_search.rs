/// Binary search
fn binary_search<'a, T: PartialOrd>(list: &'a [T], item: &'a T) -> Option<&'a T> {
    if list.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = list.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let guess = &list[mid];

        if guess == item {
            return Some(guess);
        }

        if guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found() {
        let item = 4;
        let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = binary_search(&list, &item);

        assert_eq!(*result.unwrap(), item);
    }

    #[test]
    fn not_found() {
        let item = 42;
        let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = binary_search(&list, &item);

        assert_eq!(result, None);
    }

    #[test]
    fn empty() {
        let item = 10;
        let list: [i32; 0] = [];
        let result = binary_search(&list, &item);

        assert_eq!(result, None);
    }

    #[test]
    fn boundaries() {
        let list = [1, 2, 3, 4, 5];

        assert_eq!(binary_search(&list, &list[0]).unwrap(), &list[0]);
        assert_eq!(binary_search(&list, &list[4]).unwrap(), &list[4]);
        assert_eq!(binary_search(&list, &list[2]).unwrap(), &list[2]);
    }
}
