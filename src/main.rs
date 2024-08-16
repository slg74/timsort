// timsort - stable sort that combines insertion sort and merge sort.
// 
// Smaller runs are sorted using insertion_sort(), which is efficient for small arrays.
// Larger portions of the array are handled by merge_sort().
//
// Understanding timsort:
// https://medium.com/@rylanbauermeister/understanding-timsort-191c758a42f3

use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

// constant for minimum run size
const RUN: usize = 32;
const ARRAY_SIZE: i32 = 10_000_000; // sort 10 million 32 bit ints

// insertion sort on slice of the array
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

// merge two sorted subarrays
fn merge_sort(arr: &mut [i32], left: usize, mid: usize, right: usize) {
    let left_len = mid - left + 1;
    let right_len = right - mid;

    // create temp arrays
    let mut left_arr = vec![0; left_len];
    let mut right_arr = vec![0; right_len];

    // copy data to temp arrays
    left_arr.copy_from_slice(&arr[left..=mid]);
    right_arr.copy_from_slice(&arr[(mid + 1)..=right]);

    let mut i = 0;      // left_arr index
    let mut j = 0;      // right_arr index
    let mut k = left;   // merged array index

    // merge temp arrays back into arr[left..right]
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

    // copy remaining elements of left_arr[]
    while i < left_len {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    // copy remaining elements of right_arr[]
    while j < right_len {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

// oh hey Tim, how you doin? 
fn tim_sort(arr: &mut [i32]) {
    let len = arr.len();

    // sort subarrays of size RUN
    for i in (0..len).step_by(RUN) {
        insertion_sort(arr, i, (i + RUN - 1).min(len - 1));
    }

    // start merging from size RUN (32). 
    // it will then merge to form size 64, then 128, 256, etc.
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

// parallel application of tim_sort
fn parallel_tim_sort(arr: &mut [i32]) {
    let len = arr.len();

    arr.par_chunks_mut(RUN).for_each(|chunk| {
        insertion_sort(chunk, 0, chunk.len() - 1);
    });

    let mut size = RUN;
    while size < len {
        arr.par_chunks_mut(2 * size).for_each(|chunk| {
            let chunk_len = chunk.len();
            if chunk_len > size {
                let mid = size - 1;
                let right = chunk_len - 1;
                merge_sort(chunk, 0, mid, right); 
            }
        });
        size *= 2;
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    
    // Create two identical arrays for fair comparison
    let mut arr1: Vec<i32> = (0..ARRAY_SIZE).map(|_| rng.gen_range(0..ARRAY_SIZE)).collect();
    let mut arr2 = arr1.clone();

    // Time the original tim_sort
    let start = Instant::now();
    tim_sort(&mut arr1);
    let duration = start.elapsed();
    println!("Time taken by tim_sort:                {:?}", duration);

    // Time the parallel_tim_sort
    let start = Instant::now();
    parallel_tim_sort(&mut arr2);
    let duration = start.elapsed();
    println!("Time taken by parallel_tim_sort:       {:?}", duration);

    // Verify results
    println!("First 10 elements (tim_sort):          {:?}", &arr1[..10]);
    println!("First 10 elements (parallel_tim_sort): {:?}", &arr2[..10]);
    println!("Array length (32 bit signed integers): {} elements", arr1.len());
}