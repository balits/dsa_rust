use std::usize;

pub fn quicksort<T: PartialOrd>(slice: &mut [T]) {
    let len = slice.len();
    if len < 2 {
        return;
    }

    let p = lomuto_partition(slice);
    quicksort(&mut slice[..p]);
    quicksort(&mut slice[p + 1..]);
}

fn lomuto_partition<T: PartialOrd>(slice: &mut [T]) -> usize {
    let len = slice.len();
    let pivot_idx = len - 1;
    let mut i = 0;

    for j in 0..len - 1 {
        if slice[j] <= slice[pivot_idx] {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, len - 1);
    i
}

#[cfg(test)]
mod tests {
    use super::quicksort as sort;

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
