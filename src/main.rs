use rayon::iter::{IntoParallelIterator, ParallelIterator};

const TARGET_STRING: &str = "2a0072d76ad1944ffc1c946ac644eab76f20292cbef33c43294d0bf7f872ef07";

fn main() {
	(0..u64::MAX)
		.into_par_iter()
		.for_each(|i| {
			let s = i.to_string();
			if sha256::digest(s) == TARGET_STRING {
				println!("{i}");
			}
		});
}
