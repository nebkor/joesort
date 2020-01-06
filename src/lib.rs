mod shape;
mod utils;
pub use shape::*;
pub use utils::*;

pub fn joe_sort<T: Sortable>(vals: &mut [T]) {
    moej_sort(vals, &Ordering::Less);
}

/// Naive merge sort
fn moej_sort<T: Sortable>(vals: &mut [T], order: &Ordering) {
    let vlen = vals.len();

    if vlen < 2 {
        return;
    }
    if vlen == 2 {
        if let Some(ord) = vals[1].partial_cmp(&vals[0]) {
            if ord == *order {
                vals.swap(0, 1);
            }
        }
        return;
    }

    let mid = vlen / 2;
    let (l_orig, r_orig) = vals.split_at_mut(mid);
    moej_sort(l_orig, order);
    moej_sort(r_orig, order);

    // now merge
    let mut lidx = 0;
    let mut ridx = mid;
    let mut sorted = Vec::with_capacity(vals.len());

    while lidx < mid && ridx < vlen {
        let l = vals[lidx];
        let r = vals[ridx];
        if let Some(ord) = r.partial_cmp(&l) {
            if ord == *order {
                ridx += 1;
                sorted.push(r);
            } else {
                lidx += 1;
                sorted.push(l);
            }
        } else {
            panic!()
        }
    }

    if lidx < mid {
        sorted.extend(&vals[lidx..mid]);
    }
    if ridx < vlen {
        sorted.extend(&vals[ridx..]);
    }

    vals.copy_from_slice(&sorted[..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moej_sort_ints_test() {
        let mut nums: Vec<i32> = gen_rands(10_000);

        let ushape = Shape::from_slice(&nums);

        // totally unsorted
        assert!(!ushape.sorted());
        assert!(!ushape.descending());
        assert!(!ushape.ascending());

        moej_sort(&mut nums, &std::cmp::Ordering::Less);
        // ascending order shape
        let ashape = Shape::from_slice(&nums);

        assert!(ashape.sorted());
        assert!(ashape.ascending());
        assert!(!ashape.descending());

        moej_sort(&mut nums, &std::cmp::Ordering::Greater);
        // descending order shape
        let dshape: Shape<i32> = nums.iter().copied().collect(); // use the FromIterator impl
        assert!(dshape.descending() && dshape.sorted());
        assert!(!dshape.ascending());
    }

    #[test]
    fn moej_sort_floats_test() {
        let mut nums: Vec<f32> = gen_rands(10_000);

        let ushape = Shape::from_slice(&nums);

        // totally unsorted
        assert!(!ushape.sorted());
        assert!(!ushape.descending());
        assert!(!ushape.ascending());

        moej_sort(&mut nums, &std::cmp::Ordering::Less);

        let sshape = Shape::from_slice(&nums);

        assert!(sshape.sorted());
        assert!(sshape.ascending());
        assert!(!sshape.descending());

        moej_sort(&mut nums, &std::cmp::Ordering::Greater);
        // descending order shape
        let dshape: Shape<f32> = nums.iter().copied().collect(); // use the FromIterator impl
        assert!(dshape.descending() && dshape.sorted());
        assert!(!dshape.ascending());
    }
}
