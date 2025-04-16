use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::{Duration, Instant};

pub fn bench_algorithm<F, T>(vec: &Vec<T>, sort_fn: F, num: i32, algoname: &str)
where
    F: Fn(&mut Vec<T>),
    T: Clone + PartialOrd + std::fmt::Debug,
{
    // Ensure algo works :|
    let mut vec_to_sort = vec.clone();
    sort_fn(&mut vec_to_sort);
    assert!(
        vec_to_sort.is_sorted(),
        "Vec is not sorted : {:?}",
        vec_to_sort
    );

    // Bench time
    let mut acc = Duration::ZERO;
    for _ in 0..num {
        let mut vec_to_sort = vec.clone();
        let start = Instant::now();
        sort_fn(&mut vec_to_sort);
        acc += start.elapsed();
    }
    println!(
        "{}: Average time per run: {}",
        algoname,
        format_duration(acc / (num as u32))
    );
}

fn format_duration(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{}.{:03}s", duration.as_secs(), duration.subsec_millis())
    } else if duration.as_millis() > 0 {
        format!(
            "{}.{:03}ms",
            duration.as_millis(),
            duration.subsec_micros() % 1000
        )
    } else if duration.as_micros() > 0 {
        format!(
            "{}.{:03}µs",
            duration.as_micros(),
            duration.as_nanos() % 1000
        )
    } else {
        format!("{}ns", duration.as_nanos())
    }
}

fn generate_random_array(size: usize, min: u64, max: u64, seed: Option<u64>) -> Vec<u64> {
    let mut rng = match seed {
        Some(seed_value) => StdRng::seed_from_u64(seed_value),
        None => StdRng::from_entropy(),
    };

    (0..size).map(|_| rng.gen_range(min..=max)).collect()
}

pub fn bench_algorithm_with_random_u64<F>(array_size: usize, sort_fn: F, num: u64, algoname: &str)
where
    F: Fn(&mut Vec<u64>),
{
    // Ensure algo works :|
    let mut vec_to_sort = generate_random_array(array_size, 0, array_size as u64, None);
    sort_fn(&mut vec_to_sort);
    assert!(
        vec_to_sort.is_sorted(),
        "Vec is not sorted : {:?}",
        vec_to_sort
    );

    // Bench time
    let mut acc = Duration::ZERO;
    for _ in 0..num {
        let mut vec_to_sort = generate_random_array(array_size, 0, array_size as u64, None);
        let start = Instant::now();
        sort_fn(&mut vec_to_sort);
        acc += start.elapsed();
    }
    println!(
        "{}: Average time per run: {}",
        algoname,
        format_duration(acc / (num as u32))
    );
}
