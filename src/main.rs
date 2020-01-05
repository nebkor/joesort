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
    // let mut u8ints: Vec<u8> = gen_rands(100_000);
    // let mut u32ints: Vec<u32> = gen_rands(100_000);
    let mut i32ints: Vec<i32> = gen_rands(100_000);

    // joe_sort(&mut u8ints);
    // joe_sort(&mut u32ints);
    let _ = joe_sort(&mut i32ints);

    // let mut asc_int: Vec<u32> = vec![0, 1, 2, 3, 4];
    // let mut desc_int: Vec<u32> = vec![4, 3, 2, 1, 0];
    //let mut unsorted_u32: Vec<u32> = vec![4, 0, 2, 1, 3];

    //let mut unsorted_u32: Vec<u32> = vec![4, 0, 2];

    // Joe_sort(&mut asc_int);
    // joe_sort(&mut desc_int);
}
