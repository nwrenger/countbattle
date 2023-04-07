mod algorithms;
use clap::{Parser, ValueEnum};
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_enum)]
    implementation: Implementation,
    #[arg(short, long, default_value_t = 1000)]
    length: usize,
    #[arg(short, long, default_value_t = 0)]
    seed: u64,
}

#[derive(Clone, ValueEnum)]
enum Implementation {
    Uncounted,
    Internet,
    Nils,
}

fn main() {
    let Args {
        implementation,
        length,
        seed,
    } = Args::parse();
    let uncounted = generate(length, seed);
    // let uncounted = vec![2, 1, 3, -4, -6];
    match implementation {
        Implementation::Uncounted => {
            println!("Uncounted: {uncounted:?}");
        }
        Implementation::Internet => {
            println!("{:?}", algorithms::maximum_count(uncounted));
        }
        Implementation::Nils => {
            println!("{:?}", algorithms::maximum_count_for(uncounted));
        }
    }
}

fn generate(len: usize, seed: u64) -> Vec<i32> {
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(rng.gen());
    }
    vec
}
