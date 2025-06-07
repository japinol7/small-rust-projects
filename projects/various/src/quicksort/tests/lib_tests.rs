#[cfg(test)]
use quicksort::quicksort;

mod tests {
    use super::*;

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

    #[test]
    fn test_sort_integers() {
        let mut numbers = vec![64, 34, -25, 14, 12, 22, 11, 90];
        quicksort(&mut numbers);
        assert_eq!(numbers, vec![-25, 11, 12, 14, 22, 34, 64, 90]);
    }

    #[test]
    fn test_sort_floats() {
        let mut floats = vec![2.21, 3.44, 1.41, -0.75, 1.71, 0.51, 1.83];
        quicksort(&mut floats);
        assert_eq!(floats, vec![-0.75, 0.51, 1.41, 1.71, 1.83, 2.21, 3.44]);
    }

    #[test]
    fn test_sort_strings() {
        let mut words = vec!["banana", "eye", "reload", "holster", "arrow", "blue"];
        quicksort(&mut words);
        assert_eq!(
            words,
            vec!["arrow", "banana", "blue", "eye", "holster", "reload"]
        );
    }
}
