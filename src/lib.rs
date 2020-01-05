mod shape;
pub use shape::*;

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
    use num_traits;
    use rand::prelude::*;

    use super::*;

    fn gen_rands<T: num_traits::Num>(size: usize) -> Vec<T>
    where
        rand::distributions::Standard: Distribution<T>,
    {
        let mut v: Vec<T> = Vec::with_capacity(size);
        let mut gen = thread_rng();
        for _ in 0..size {
            v.push(gen.gen())
        }
        v
    }

    #[test]
    fn shape_sorted_test() {
        let asc_int: Vec<u32> = vec![0, 1, 2, 3, 4];
        let desc_int: Vec<u32> = vec![4, 3, 2, 1, 0];
        let unsorted_u32: Vec<u32> = vec![4, 0, 2, 1, 3];

        let a_shape = Shape::from_slice(&asc_int);
        let d_shape = Shape::from_slice(&desc_int);
        let u_shape = Shape::from_slice(&unsorted_u32);

        assert!(a_shape.sorted());
        assert!(d_shape.sorted());
        assert!(!u_shape.sorted());

        assert_eq!(2.0, a_shape.mean());
        assert_eq!(2.0, d_shape.mean());
        assert_eq!(2.0, u_shape.mean());
    }

    #[test]
    fn moej_sort_ints_test() {
        let mut nums: Vec<i32> = gen_rands(10_000);

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
    }
}
