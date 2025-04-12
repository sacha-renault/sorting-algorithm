use std::time::{Duration, Instant};

pub fn bench_algorithm<F, T>(vec: &Vec<T>, sort_fn: F, num: i32, algoname: &str)
where
    F: Fn(&mut Vec<T>),
    T: Clone + PartialOrd,
{
    // Ensure algo works :|
    let mut vec_to_sort = vec.clone();
    sort_fn(&mut vec_to_sort);
    assert!(vec_to_sort.is_sorted());

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
