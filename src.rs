use usearch::{Index, IndexOptions, MetricKind, ScalarKind, new_index};
use chrono::Local;
use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    let options = IndexOptions {
        dimensions: 512,
        metric: MetricKind::L2sq,
        quantization: ScalarKind::F32,
        connectivity: 0, // Let it be auto-tuned
        expansion_add: 0,
        expansion_search: 0,
        multi: false,
    };

    let index: Index = new_index(&options).expect("Failed to create index");

    let num_vectors = 50_000 as usize;
    let dimensions = 512;

    index.reserve(num_vectors).expect("Failed to reserve space");

    let mut vec = vec![0f32; dimensions];
    for id in 0..num_vectors {
        for v in vec.iter_mut() {
            *v = rng.random::<f32>();
        }
        index.add(id as u64, &vec).expect("Failed to add vector");
    }

    assert_eq!(index.size(), num_vectors);

    // Create a random query vector
    let mut query = vec![0f32; dimensions];
    for v in query.iter_mut() {
        *v = rng.random::<f32>();
    }

    println!("Search start: {}", Local::now());
    let _results = index.search(&query, 10).expect("Search failed");
    println!("Search end: {}", Local::now());

}
