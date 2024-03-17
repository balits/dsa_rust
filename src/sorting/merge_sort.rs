use std::fmt::Debug;

#[allow(dead_code)]
pub fn merge_sort<T>(slice: &mut [T])
where
    T: PartialOrd + Copy,
{
    if slice.len() < 2 {
        return;
    }
    let mid = slice.len() / 2;

    merge_sort(&mut slice[..mid]);
    merge_sort(&mut slice[mid..]);

    let len = slice.len();
    let (mut i, mut j) = (0, mid);
    let mut result = Vec::with_capacity(len);

    while i < mid && j < len {
        if slice[i] < slice[j] {
            result.push(slice[i]);
            i += 1;
        } else {
            result.push(slice[j]);
            j += 1;
        }
    }

    if i < mid {
        result.extend_from_slice(&slice[i..mid]);
    } else if j < len {
        result.extend_from_slice(&slice[j..len]);
    }

    slice.copy_from_slice(&result[..]);
}
#[cfg(test)]
mod tests {
    use super::merge_sort as sort;

    #[test]
    fn test_sort_empty() {
        let mut arr: [i32; 0] = [];
        sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_sort_single_element() {
        let mut arr = [42];
        sort(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_sort_two_element() {
        let mut arr = [42, 24];
        sort(&mut arr);
        assert_eq!(arr, [24, 42]);
    }

    #[test]
    fn test_sort_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_reverse_sorted() {
        let mut arr = [5, 4, 3, 2, 1];
        sort(&mut arr);
        println!("{:?}", &arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_unsorted() {
        let mut arr = [5, 2, 4, 1, 3];
        sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_strings() {
        let mut arr = ["banana", "apple", "cherry", "date", "blueberry"];
        sort(&mut arr);
        assert_eq!(arr, ["apple", "banana", "blueberry", "cherry", "date"]);
    }
}
