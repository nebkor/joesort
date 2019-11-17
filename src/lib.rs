use num_traits::{Num, ToPrimitive};

struct Shape<T: ToPrimitive> {
    min: T,
    max: T,
    mean: T,
    std_dev: T,
    median: T,
}

impl<T: ToPrimitive> Shape<T> {
    fn new(min: T, max: T, mean: T, std_dev: T, median: T) -> Shape<T> {
        Shape {
            min,
            max,
            mean,
            std_dev,
            median,
        }
    }
}

fn get_shape<T: ToPrimitive + Num + PartialOrd>(vals: &mut [T]) -> Shape<T> {
    let mut min = vals[0];
    let mut max = vals[0];
    let mut mean = vals[0];
    let mut median = vals[0];
    let mut std_dev = T::zero();

    Shape::new(min, max, mean, std_dev, median)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
