use crossbeam_utils::thread;
use ndarray::{Array, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use num_traits::Zero;
use std::collections::HashMap;
use std::fmt::Display;
use std::time::SystemTime;

struct BlockManager<T>
where
    T: Display + Send + Sync + Clone + Zero,
{
    blocks: HashMap<isize, Array2<T>>,
}

impl<T> BlockManager<T>
where
    T: Display + Send + Sync + Clone + Zero,
{
    fn new(blocks: HashMap<isize, Array2<T>>) -> Self {
        Self { blocks }
    }

    fn sum(&self) {
        for (key, value) in &self.blocks {
            println!("sum of {} is {}", key, value.sum());
        }
    }

    fn parallel_sum(&self) {
        thread::scope(|s| {
            for (key, value) in &self.blocks {
                s.spawn(move |_| println!("sum of {} is {}", key, value.sum()));
            }
        })
        .unwrap();
    }
}

fn main() {
    let n = 1_000_000;
    let k = 100;
    let data = HashMap::from([
        (0, Array::random((n, k), Uniform::new(0., 10.))),
        (1, Array::random((n, k), Uniform::new(0., 10.))),
        (2, Array::random((n, k), Uniform::new(0., 10.))),
        (3, Array::random((n, k), Uniform::new(0., 10.))),
        (4, Array::random((n, k), Uniform::new(0., 10.))),
        (5, Array::random((n, k), Uniform::new(0., 10.))),
        (6, Array::random((n, k), Uniform::new(0., 10.))),
        (7, Array::random((n, k), Uniform::new(0., 10.))),
        (8, Array::random((n, k), Uniform::new(0., 10.))),
        (9, Array::random((n, k), Uniform::new(0., 10.))),
    ]);

    let mgr = BlockManager::new(data);

    let serial_start = SystemTime::now();
    mgr.sum();
    let serial_end = SystemTime::now();
    let parallel_start = SystemTime::now();
    mgr.parallel_sum();
    let parallel_end = SystemTime::now();

    let serial_duration = serial_end.duration_since(serial_start).unwrap();
    let parallel_duration = parallel_end.duration_since(parallel_start).unwrap();
    println!(
        "serial implementation took {} milliseconds",
        serial_duration.as_millis()
    );
    println!(
        "parallel implementation took {} milliseconds",
        parallel_duration.as_millis()
    );
}
