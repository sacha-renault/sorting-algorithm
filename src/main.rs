mod algorithms;
#[allow(unused)]
mod bench;

use algorithms::heap_sort::heap_sort;
use algorithms::merge_sort::merge_sort;
use algorithms::quick_sort::quick_sort;
use bench::bench_algorithm_with_random_u64;

fn main() {
    let n_tests = 1000;
    let array_size = 1000;

    bench_algorithm_with_random_u64(array_size, |vec| vec.sort(), n_tests, "Built-in Rust Sort");
    bench_algorithm_with_random_u64(
        array_size,
        |vec| vec.sort_unstable(),
        n_tests,
        "Built-in Rust Sort Unstable",
    );
    bench_algorithm_with_random_u64(
        array_size,
        |vec| merge_sort(vec, |a, b| a.cmp(&b)),
        n_tests,
        "My Merge Sort",
    );
    bench_algorithm_with_random_u64(
        array_size,
        |vec| quick_sort(vec, |a, b| a.cmp(&b)),
        n_tests,
        "My Quick Sort",
    );
    bench_algorithm_with_random_u64(
        array_size,
        |vec| heap_sort(vec, |a, b| a.cmp(&b)),
        n_tests,
        "My Heap Sort",
    );
}
