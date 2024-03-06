pub mod sha256;
pub mod sha256_iter;

use sha256::sha256;
use sha256_iter::sha256_iter;

use criterion::criterion_main;

criterion_main!(sha256, sha256_iter);
