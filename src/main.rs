mod algorithms;
mod bench;

use algorithms::merge_sort::merge_sort;
use bench::bench_algorithm;

fn main() {
    let vec = vec![5, 3, 8, 1, 2];

    bench_algorithm(&vec, |vec| vec.sort(), 1000, "Built-in Rust Sort");
    bench_algorithm(
        &vec,
        |vec| merge_sort(vec, |a, b| a.cmp(&b)),
        1000,
        "My Merge Sort",
    );
    println!("Hello, world! {:?}", vec);
}
