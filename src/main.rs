mod algorithms;
mod bench;

use algorithms::heap_sort::heap_sort;
use algorithms::merge_sort::merge_sort;
use algorithms::quick_sort::quick_sort;
use bench::bench_algorithm;

fn main() {
    let vec = vec![
        42, 17, 89, 55, 23, 76, 11, 99, 63, 28, 5, 70, 33, 81, 46, 14, 92, 57, 25, 78, 9, 64, 37,
        83, 50, 21, 67, 39, 85, 52, 19, 73, 45, 88, 30, 2, 61, 34, 79, 47, 15, 68, 41, 86, 54, 27,
        75, 48, 90, 32, 7, 62, 35, 84, 51, 18, 71, 44, 87, 29, 3, 66, 38, 82, 49, 16, 72, 43, 91,
        31, 8, 65, 36, 77, 53, 22, 74, 40, 80, 26, 12, 69, 94, 58, 20, 60, 100, 1, 59, 96, 13, 95,
        4, 98, 24, 97, 10, 56, 6, 93,
    ];

    bench_algorithm(&vec, |vec| vec.sort(), 1000, "Built-in Rust Sort");
    bench_algorithm(
        &vec,
        |vec| merge_sort(vec, |a, b| a.cmp(&b)),
        1000,
        "My Merge Sort",
    );
    bench_algorithm(
        &vec,
        |vec| quick_sort(vec, |a, b| a.cmp(&b)),
        1000,
        "My Quick Sort",
    );
    bench_algorithm(
        &vec,
        |vec| heap_sort(vec, |a, b| a.cmp(&b)),
        1000,
        "My Heap Sort",
    );
}
