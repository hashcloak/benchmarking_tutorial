pub mod sha256;
pub mod sha2562;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sha256::benches2;
use sha2562::benches;

criterion_main!(benches, benches2);
