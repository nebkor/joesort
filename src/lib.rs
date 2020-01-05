use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::iter::{FromIterator, IntoIterator};

use num_traits::{FromPrimitive, ToPrimitive, Zero};

pub trait Sortable:
    PartialOrd + FromPrimitive + ToPrimitive + Debug + Copy + Zero + Display
{
}
impl<T> Sortable for T where
    T: PartialOrd + PartialEq + FromPrimitive + ToPrimitive + Debug + Copy + Zero + Display
{
}

#[derive(Debug)]
pub struct Shape<T> {
    min: Option<T>,
    max: Option<T>,
    last_v: T,
    mean: f64,
    median: T,
    a_sorted: bool,
    d_sorted: bool,
    variance: f64,
    size: usize,
}

// Strongly inspired by, and heavily evolved from
// https://docs.rs/streaming-stats/0.2.2/stats/struct.OnlineStats.html
impl<T: Sortable> Shape<T> {
    /// constructs empty Shape
    pub fn new() -> Self {
        Shape {
            min: None,
            max: None,
            last_v: T::zero(),
            mean: 0.0,
            median: T::zero(),
            a_sorted: true,
            d_sorted: true,
            variance: 0.0,
            size: 0,
        }
    }

    /// Initializes all fields from a slice
    pub fn from_slice(samples: &[T]) -> Self {
        samples.iter().copied().collect()
    }

    /// Return the current mean.
    pub fn mean(&self) -> f64 {
        self.mean
    }

    /// Return the current standard deviation.
    pub fn stddev(&self) -> f64 {
        self.variance.sqrt()
    }

    /// Return the current variance.
    pub fn variance(&self) -> f64 {
        self.variance
    }

    /// Return current max
    pub fn max(&self) -> Option<T> {
        self.max
    }

    pub fn min(&self) -> Option<T> {
        self.min
    }

    pub fn median(&self) -> T {
        self.median
    }

    pub fn sorted(&self) -> bool {
        self.a_sorted || self.d_sorted
    }

    pub fn ascending(&self) -> bool {
        self.a_sorted
    }

    pub fn descending(&self) -> bool {
        self.d_sorted
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /// Add a new sample.
    pub fn add(&mut self, sample: T) {
        let fsample = sample.to_f64().unwrap();
        // Taken from: http://goo.gl/JKeqvj
        // See also: http://goo.gl/qTtI3V
        let oldmean = self.mean;
        let prevq = self.variance * (self.size as f64);

        let old_lastv: T;
        if self.size > 0 {
            old_lastv = self.last_v;
        } else {
            old_lastv = sample;
        }
        self.last_v = sample;

        self.size += 1;
        self.mean += (fsample - oldmean) / (self.size as f64);
        self.variance = (prevq + (fsample - oldmean) * (fsample - self.mean)) / (self.size as f64);

        if let Some(m) = self.min {
            if sample < m {
                self.min.replace(sample);
            }
        } else {
            self.min.replace(sample);
        }

        if let Some(m) = self.max {
            if sample > m {
                self.max.replace(sample);
            }
        } else {
            self.max.replace(sample);
        }

        if self.a_sorted || self.d_sorted {
            if let Some(ord) = self.last_v.partial_cmp(&old_lastv) {
                match ord {
                    Ordering::Less => {
                        self.a_sorted = false;
                    }
                    Ordering::Greater => {
                        self.d_sorted = false;
                    }
                    _ => {}
                }
            }
        }
    }

    /// Add a new NULL value to the population.
    ///
    /// This increases the population size by `1`, but does not affect min, max, or sorted status
    pub fn add_null(&mut self) {
        let oldmean = self.mean;
        let prevq = self.variance * (self.size as f64);
        self.size += 1;
        self.mean += oldmean / (self.size as f64);
        self.variance = ((prevq + oldmean) * self.mean) / (self.size as f64);

        self.size += 1
    }

    /// Returns the number of data points.
    pub fn len(&self) -> usize {
        self.size
    }
}

impl<T: Sortable> FromIterator<T> for Shape<T> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Shape<T> {
        let mut v = Shape::new();
        v.extend(it);
        v
    }
}

impl<T: Sortable> Extend<T> for Shape<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, it: I) {
        for sample in it {
            self.add(sample)
        }
    }
}

pub fn joe_sort<T: Sortable>(vals: &mut [T]) {
    moej_sort(vals, &Ordering::Less);
}

/// Naive merge sort
fn moej_sort<T: Sortable>(vals: &mut [T], order: &Ordering) {
    if vals.len() < 2 {
        return;
    }

    let mid = vals.len() / 2;
    let (l_orig, r_orig) = vals.split_at_mut(mid);
    moej_sort(l_orig, order);
    moej_sort(r_orig, order);

    // now merge
    let mut lidx = 0;
    let mut ridx = mid;
    let mut sorted = Vec::with_capacity(vals.len());

    while lidx < mid && ridx < vals.len() {
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
        }
    }

    if lidx < mid {
        sorted.extend(&vals[lidx..mid]);
    }
    if ridx < vals.len() {
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
    fn moej_sort_test() {
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
}
