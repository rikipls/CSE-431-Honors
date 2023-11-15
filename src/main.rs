use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use std::collections::{HashSet, BTreeSet};
use std::error::Error;
use std::time::Instant;
use std::fs::File;
use std::io::Write;

const OUT_DIR: &str = "out";
const SEED: u64 = 431;
const NUM_ELEMENTS: usize = 10_000_000;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);

    // Generate the random data
    let data: Vec<usize> = (0..NUM_ELEMENTS).map(|_| rng.gen_range(0..usize::MAX)).collect();

    // Copies
    let mut hashmap_time: Vec<u128> = vec![0; NUM_ELEMENTS];
    let mut btreemap_time: Vec<u128>  = vec![0; NUM_ELEMENTS];
    let mut total_hashmap_time = 0.0;
    let mut total_btreemap_time = 0.0;

    let mut hash_map= HashSet::<usize>::new();
    let mut btree_map = BTreeSet::<usize>::new();

    let mut num_elements_processed = NUM_ELEMENTS;

    for i in 0..NUM_ELEMENTS {
        let val = data[i];

        // HashMap
        let now_hashmap = Instant::now();
        hash_map.insert(val);
        let hash_elapsed = now_hashmap.elapsed();
        hashmap_time[i] = hash_elapsed.as_nanos();
        total_hashmap_time += hash_elapsed.as_secs_f64();

        // BTreemap
        let now_btreemap = Instant::now();
        btree_map.insert(val);
        let btree_elapsed = now_btreemap.elapsed();
        btreemap_time[i] = btree_elapsed.as_nanos();
        total_btreemap_time += btree_elapsed.as_secs_f64();

        // Stop after total elapsed time exceeds 3 seconds
        if total_btreemap_time >= 3.0 || total_hashmap_time >= 3.0 {
            num_elements_processed = i;
            break;
        }
    }

    let nonzero_hashmap_times_in_ns= &hashmap_time[..num_elements_processed];
    let nonzero_btreemap_times_in_ns= &btreemap_time[..num_elements_processed];

    let hash_times_str: Vec<String> = nonzero_hashmap_times_in_ns.iter().map(|n| format!("{n}")).collect();
    let tree_times_str: Vec<String> = nonzero_btreemap_times_in_ns.iter().map(|n| format!("{n}")).collect();

    let mut f_hash = File::create(format!("{OUT_DIR}/hash.txt"))?;
    let mut f_tree = File::create(format!("{OUT_DIR}/tree.txt"))?;

    writeln!(f_hash, "{}", hash_times_str.join("\n"))?;
    writeln!(f_tree, "{}", tree_times_str.join("\n"))?;

    Ok(())
}