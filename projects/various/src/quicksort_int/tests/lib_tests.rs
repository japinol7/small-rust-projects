#[cfg(test)]
use quicksort_int::quicksort;

mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut numbers = vec![64, 34, -25, 14, 12, 22, 11, 90];
        quicksort(&mut numbers);
        assert_eq!(numbers, vec![-25, 11, 12, 14, 22, 34, 64, 90]);
    }

    #[test]
    fn test_empty_array() {
        let mut numbers: Vec<i32> = vec![];
        quicksort(&mut numbers);
        assert_eq!(numbers, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut numbers = vec![42];
        quicksort(&mut numbers);
        assert_eq!(numbers, vec![42]);
    }
}
