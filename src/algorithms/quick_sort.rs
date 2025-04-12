use std::cmp::Ordering;

pub fn quick_sort<T, F>(a: &mut [T], cmp_fn: F)
where
    T: Copy + PartialOrd,
    F: Fn(T, T) -> Ordering + Copy,
{
    if a.len() <= 1 {
        return;
    }

    let pivot_index = partition(a, cmp_fn);

    // Sort the left part
    quick_sort(&mut a[0..pivot_index], cmp_fn);

    // Sort the right part (pivot element is already in place)
    quick_sort(&mut a[pivot_index + 1..], cmp_fn);
}

/// Place all element that are less than pivot before pivot
/// Return the final position of the pivot
fn partition<T, F>(a: &mut [T], cmp_fn: F) -> usize
where
    T: Copy + PartialOrd,
    F: Fn(T, T) -> Ordering + Copy,
{
    // Get the pivot and move it to the end of the array
    let init_pivot_index = median_of_three(a);
    a.swap(init_pivot_index, a.len() - 1);
    let pivot_index = a.len() - 1;
    let pivot = a[pivot_index];

    // track how many elt are less than pivot
    let mut less_than_pivot = 0;

    // iterate for all element except pivot
    for j in 0..a.len() {
        if j != pivot_index && matches!(cmp_fn(a[j], pivot), Ordering::Less) {
            // moves element that are less than pivot in the begenning of the array
            // Which pushes naturally bigger element to the right
            a.swap(less_than_pivot, j);
            less_than_pivot += 1;
        }
    }
    // Here, all the element less than pivot are placed unsorted
    // At the begening of the array, then from [pivot_index..-1] we can find
    // all the biggest element. We just need  to place the pivot at the final position
    // So all < pivot can be found [0..less_than_pivot] and [less_than_pivot+1..] has all
    // element greater than pivot
    a.swap(less_than_pivot, pivot_index);
    less_than_pivot
}

fn median_of_three<T: Copy + PartialOrd>(a: &[T]) -> usize {
    if a.len() <= 1 {
        return 0;
    }

    let first_idx = 0;
    let middle_idx = a.len() / 2;
    let last_idx = a.len() - 1;

    let first = a[first_idx];
    let middle = a[middle_idx];
    let last = a[last_idx];

    // If first is between middle and last, return first's index
    if (first >= middle && first <= last) || (first <= middle && first >= last) {
        return first_idx;
    }
    // If middle is between first and last, return middle's index
    else if (middle >= first && middle <= last) || (middle <= first && middle >= last) {
        return middle_idx;
    }
    // Otherwise, last must be the median
    else {
        return last_idx;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_test() {
        let mut vec = vec![5, 3, 8, 1, 2];
        quick_sort(&mut vec, |a, b| a.cmp(&b));
        assert!(vec.is_sorted(), "Sorting algorithm isn't working well ...")
    }
}
