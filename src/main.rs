use joesort::*;
use rand::prelude::*;

fn gen_rands<T>(size: usize) -> Vec<T>
where
    rand::distributions::Standard: Distribution<T>,
{
    let mut rng = thread_rng();
    rand::distributions::Standard
        .sample_iter(&mut rng)
        .take(size)
        .collect()
}

fn main() {
    let mut i8ints: Vec<i8> = gen_rands(100);
    let ushape = Shape::from_slice(&i8ints[..]);
    println!("{:?}\n{:?}\n", &i8ints, ushape);

    joe_sort(&mut i8ints);
    let sshape = Shape::from_slice(&i8ints[..]);
    println!("{:?}\n{:?}", &i8ints, sshape);
}
