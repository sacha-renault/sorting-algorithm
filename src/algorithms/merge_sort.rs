//! Basic merge sort
//! Check out: https://en.wikipedia.org/wiki/Merge_sort

use std::cmp::Ordering;

pub fn merge_sort<T, F>(a: &mut [T], cmp_fn: F)
where
    T: Copy + PartialOrd,
    F: Fn(T, T) -> Ordering + Copy,
{
    let mut b = a.to_vec();
    top_down_split_merge(&mut b, a, 0, a.len(), cmp_fn);
}

fn top_down_split_merge<T, F>(a: &mut [T], b: &mut [T], start: usize, end: usize, cmp_fn: F)
where
    T: Copy + PartialOrd,
    F: Fn(T, T) -> Ordering + Copy,
{
    // chunk is small enough, can stop recursion
    if end - start <= 1 {
        return;
    }

    // split the run longer than 1 item into halves
    let mid = (start + end) / 2;

    // Call recursively on both sides
    top_down_split_merge(b, a, start, mid, cmp_fn);
    top_down_split_merge(b, a, mid, end, cmp_fn);

    // merge the result array from B to A
    top_down_merge(a, b, start, mid, end, cmp_fn);
}

fn top_down_merge<T, F>(a: &mut [T], b: &mut [T], start: usize, mid: usize, end: usize, cmp_fn: F)
where
    T: Copy + PartialOrd,
    F: Fn(T, T) -> Ordering + Copy,
{
    let (mut i, mut j) = (start, mid);

    for k in start..end {
        if i < mid && (j >= end || matches!(cmp_fn(a[i], a[j]), Ordering::Equal | Ordering::Less)) {
            b[k] = a[i];
            i += 1;
        } else {
            b[k] = a[j];
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_test() {
        let mut vec = vec![5, 3, 8, 1, 2];
        merge_sort(&mut vec, |a, b| a.cmp(&b));
        assert!(vec.is_sorted(), "Sorting algorithm isn't working well ...")
    }
}
