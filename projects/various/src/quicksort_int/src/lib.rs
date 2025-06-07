/// Partitions an array segment and returns the pivot index
pub fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let mut index = low as isize - 1;
    let pivot_element = arr[high];

    for i in low..high {
        if arr[i] <= pivot_element {
            index += 1;
            arr.swap(index as usize, i);
        }
    }

    arr.swap((index + 1) as usize, high);
    (index + 1) as usize
}

/// Sorts a range within the array using quicksort algorithm
pub fn quicksort_range(arr: &mut [i32], low: usize, high: usize) {
    if arr.len() <= 1 {
        return;
    }

    if low < high {
        let pivot = partition(arr, low, high);

        if pivot > 0 {
            quicksort_range(arr, low, pivot.saturating_sub(1));
        }

        quicksort_range(arr, pivot + 1, high);
    }
}

/// Sorts an array using quicksort algorithm
pub fn quicksort(arr: &mut [i32]) -> &mut [i32] {
    if !arr.is_empty() {
        let last_index = arr.len() - 1;
        quicksort_range(arr, 0, last_index);
    }
    arr
}
