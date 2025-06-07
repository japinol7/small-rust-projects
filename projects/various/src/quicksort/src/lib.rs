use std::cmp::PartialOrd;

/// Partitions an array segment and returns the pivot index
pub fn partition<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;

    for j in low..high {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot);
    i
}

/// Sorts a range within the array using quicksort algorithm
pub fn quicksort_range<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) {
    if arr.len() <= 1 {
        return;
    }

    if low < high {
        let pivot = partition(arr, low, high);

        if pivot > 0 {
            quicksort_range(arr, low, pivot - 1);
        }

        quicksort_range(arr, pivot + 1, high);
    }
}

/// Sorts an array using quicksort algorithm
pub fn quicksort<T: PartialOrd + Clone>(arr: &mut [T]) {
    if !arr.is_empty() {
        let last_index = arr.len() - 1;
        quicksort_range(arr, 0, last_index);
    }
}
