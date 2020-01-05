use rand::prelude::*;

pub fn gen_rands<T>(size: usize) -> Vec<T>
where
    rand::distributions::Standard: Distribution<T>,
{
    let mut rng = thread_rng();
    rand::distributions::Standard
        .sample_iter(&mut rng)
        .take(size)
        .collect()
}
