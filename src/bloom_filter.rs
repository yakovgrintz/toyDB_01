use bincode;
use murmur3::murmur3_32;
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Cursor;
use std::marker::PhantomData;

struct BloomFilter<T> {
    bit_vector: Vec<bool>,
    size: usize,
    num_of_functions: usize,
    phantom: PhantomData<T>,
}

impl<T: Serialize + Hash> BloomFilter<T> {
    fn new(n: f32, error_percent: f32) -> Self {
        let ln2 = 2.0_f32.ln();
        let size = -(n * (error_percent.ln())) / (ln2.powf(2.0));
        let num_of_functions = ((size / n) * (ln2)) as usize;

        BloomFilter {
            bit_vector: vec![false; size as usize],
            size: size as usize,
            num_of_functions,
            phantom: PhantomData,
        }
    }
    fn hash(&self, item: &T) -> Vec<usize> {
        let mut bit_position = Vec::new();
        let bit_stream = bincode::serialize(item).unwrap();
        let mut buffer = Cursor::new(&bit_stream);
        for i in 0..self.num_of_functions {
            let hash1 = murmur3_32(&mut buffer, 0).unwrap() as usize;
            let hash2 = Self::fnv1a_32(item) as usize;
            let result = (hash1.wrapping_add(i * hash2)) % self.size;

            bit_position.push(result);
        }
        bit_position
    }
    fn add(&mut self, item: &T) {
        let positions = self.hash(item);
        for pos in positions {
            self.bit_vector[pos] = true;
        }
    }
    fn check(&self, item: &T) -> bool {
        let positions = self.hash(item);
        positions.iter().all(|&pos| self.bit_vector[pos])
    }

    fn fnv1a_32(key: &T) -> u32 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as u32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Hash, Serialize)]
    struct TestItem {
        id: u32,
        value: String,
    }

    // Helper function to create a test item
    fn create_item(id: u32, value: &str) -> TestItem {
        TestItem {
            id,
            value: value.to_string(),
        }
    }

    #[test]
    fn bloom_filter_initialization() {
        let n = 100_f32; // expected number of items
        let error_percent = 0.01_f32; // desired error rate
        let filter = BloomFilter::<TestItem>::new(n, error_percent);

        let expected_size = (-(n * error_percent.ln()) / (2.0_f32.ln().powi(2))) as usize;
        let expected_num_of_functions = ((expected_size as f32 / n) * 2.0_f32.ln()) as usize;

        assert_eq!(filter.size, expected_size);
        assert_eq!(filter.num_of_functions, expected_num_of_functions);
        assert_eq!(filter.bit_vector.len(), expected_size);
    }

    #[test]
    fn add_and_check_element() {
        let mut filter = BloomFilter::<TestItem>::new(100_f32, 0.01_f32);
        let item = create_item(1, "test");

        filter.add(&item);

        assert!(filter.check(&item));
    }

    #[test]
    fn check_nonexistent_element() {
        let filter = BloomFilter::<TestItem>::new(100_f32, 0.01_f32);
        let item = create_item(1, "test");
        let non_existent_item = create_item(2, "nonexistent");

        // Not added the item, should ideally be false, acknowledging potential false positives
        assert!(!filter.check(&non_existent_item));
    }

    #[test]
    fn false_positive_rate() {
        let mut filter = BloomFilter::<TestItem>::new(1000_f32, 0.01_f32);
        let mut false_positives = 0;
        let mut trials = 10000;

        for id in 0..500 {
            // Add 500 items
            let item = create_item(id, &format!("item{}", id));
            filter.add(&item);
        }

        for id in 500..(500 + trials) {
            // Check 10000 different items
            let non_existent_item = create_item(id, &format!("item{}", id));
            if filter.check(&non_existent_item) {
                false_positives += 1;
            }
        }

        let false_positive_rate = false_positives as f32 / trials as f32;
        println!("False positive rate: {}", false_positive_rate);

        // The actual rate might slightly vary, but should be close to the desired rate
        assert!(false_positive_rate <= 0.01 + 0.005); // Allowing some margin
    }
}
