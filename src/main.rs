use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use std::collections::{HashSet, BTreeSet, hash_map};
use std::error::Error;
use std::time::Instant;
use std::fs::File;
use std::hash::BuildHasher;
use std::io::Write;

const OUT_DIR: &str = "out";
const SEED: u64 = 431;
const NUM_ELEMENTS: usize = 30_000_000;

fn main() -> Result<(), Box<dyn Error>> {
    // Pseudorandom data
    run::<hash_map::RandomState>("sip13", true)?;
    run::<ahash::RandomState>("ahash", true)?;

    // All elements are distinct
    run::<hash_map::RandomState>("sip13", false)?;
    run::<ahash::RandomState>("ahash", false)
}

fn run<Hasher: Default + BuildHasher>(out_path: &str, random_data: bool) -> Result<(), Box<dyn Error>> {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    // Either use pseudorandom data or have data[i] == i for unique elements
    let data: Vec<usize> = if random_data {
        (0..NUM_ELEMENTS).map(|_| rng.gen_range(0..usize::MAX)).collect()
    } else {
        (0..NUM_ELEMENTS).collect()
    };

    // Copies
    let mut hashset_time: Vec<u128> = vec![0; NUM_ELEMENTS];
    let mut btreeset_time: Vec<u128> = vec![0; NUM_ELEMENTS];
    let mut total_hashset_time = 0.0;
    let mut total_btreeset_time = 0.0;

    let mut hash_set: HashSet<usize, Hasher> = HashSet::default();
    let mut btree_set: BTreeSet<usize> = BTreeSet::new();

    let mut num_elements_processed = NUM_ELEMENTS;

    for i in 0..NUM_ELEMENTS {
        let val = data[i];

        // hashset
        let now_hashset = Instant::now();
        hash_set.insert(val);
        let hash_elapsed = now_hashset.elapsed();
        hashset_time[i] = hash_elapsed.as_nanos();
        total_hashset_time += hash_elapsed.as_secs_f64();

        // btreeset
        let now_btreeset = Instant::now();
        btree_set.insert(val);
        let btree_elapsed = now_btreeset.elapsed();
        btreeset_time[i] = btree_elapsed.as_nanos();
        total_btreeset_time += btree_elapsed.as_secs_f64();

        // Stop after total elapsed time exceeds 3 seconds
        if total_btreeset_time >= 3.0 || total_hashset_time >= 3.0 {
            num_elements_processed = i;
            break;
        }
    }

    let nonzero_hashset_times_in_ns= &hashset_time[..num_elements_processed];
    let nonzero_btreeset_times_in_ns= &btreeset_time[..num_elements_processed];

    let hash_times_str: Vec<String> = nonzero_hashset_times_in_ns.iter().map(|n| format!("{n}")).collect();
    let tree_times_str: Vec<String> = nonzero_btreeset_times_in_ns.iter().map(|n| format!("{n}")).collect();

    let f_type = if random_data { "random" } else { "fixed"};

    let mut f_hash = File::create(format!("{OUT_DIR}/{out_path}-{f_type}-hash.txt"))?;
    let mut f_tree = File::create(format!("{OUT_DIR}/{out_path}-{f_type}-tree.txt"))?;

    writeln!(f_hash, "{}", hash_times_str.join("\n"))?;
    writeln!(f_tree, "{}", tree_times_str.join("\n"))?;

    Ok(())
}