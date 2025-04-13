use std::cmp::Ordering;

pub fn heap_sort<T, F>(a: &mut [T], cmp_fn: F)
where
    T: Copy + PartialOrd,
    F: Fn(T, T) -> Ordering + Copy,
{
    let count = a.len();
    let mut start_index = count / 2;
    let mut end_index = count;

    while end_index > 1 {
        if start_index > 0 {
            // Heap construction
            start_index -= 1;
        } else {
            // heap extraction
            end_index -= 1;
            a.swap(end_index, 0);
        }

        // SiftDown procedure
        let mut root = start_index;

        // Get l child
        let mut left_child = 2 * root + 1;
        while left_child < end_index {
            // If there is a right child and that child is greater
            let mut child = left_child;

            if child + 1 < end_index && matches!(cmp_fn(a[child], a[child + 1]), Ordering::Less) {
                child += 1;
            }

            if matches!(cmp_fn(a[root], a[child]), Ordering::Less) {
                a.swap(root, child);
                root = child;
                left_child = 2 * root + 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heap_sort_test() {
        let mut vec = vec![5, 3, 8, 1, 2];
        heap_sort(&mut vec, |a, b| a.cmp(&b));
        assert!(vec.is_sorted(), "Sorting algorithm isn't working well ...")
    }
}
