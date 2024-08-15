// timsort - 0(n log n) best case, 0(n) worst case. combines insertion sort and merge sort.

// Smaller runs are sorted using insertion_sort(), which is efficient for small arrays.
// Larger portions of the array are handled by merge_sort().

const RUN: usize = 32;

fn insertion_sort(arr: &mut [i32], left: usize, right: usize) {
    for i in (left + 1)..=right {
        let temp = arr[i];
        let mut j = i;
        while j > left && arr[j - 1] > temp {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = temp;
    }
}

fn merge_sort(arr: &mut [i32], left: usize, mid: usize, right: usize) {
    let left_len = mid - left + 1;
    let right_len = right - mid;
    let mut left_arr = vec![0; left_len];
    let mut right_arr = vec![0; right_len];

    left_arr.copy_from_slice(&arr[left..=mid]);
    right_arr.copy_from_slice(&arr[(mid + 1)..=right]);

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    while i < left_len && j < right_len {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < left_len {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < right_len {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

fn tim_sort(arr: &mut [i32]) {
    let len = arr.len();

    for i in (0..len).step_by(RUN) {
        insertion_sort(arr, i, (i + RUN - 1).min(len - 1));
    }

    let mut size = RUN;
    while size < len {
        for left in (0..len).step_by(2 * size) {
            let mid = (left + size - 1).min(len - 1);
            let right = (left + 2 * size - 1).min(len - 1);
            merge_sort(arr, left, mid, right);
        }
        size *= 2;
    }
}

fn main() {
    let mut arr = vec![10, 3, 2, 19, 7, 15, 23, 13, 1];
    println!("Before sorting: {:?}", arr);
    tim_sort(&mut arr);
    println!("After sorting: {:?}", arr);
}