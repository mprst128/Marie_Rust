//! # Cache LRU (Least Recently Used)
//!
//! Cette bibliothèque fournit une implémentation d'un cache LRU optimisé en Rust.

pub mod cache;
pub mod errors;

pub use cache::Cache;
pub use errors::{CacheError, CacheResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = Cache::new(3);
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));
        // Cache == [B, C, D]

        let my_value = cache.get(&"A");
        assert_eq!(my_value, None);

        let my_value = cache.get(&"D");
        assert_eq!(my_value, Some(&String::from("value_d")));

        let my_value = cache.get(&"B");
        assert_eq!(my_value, Some(&String::from("value_b")));

        let my_value = cache.get(&"C");
        assert_eq!(my_value, Some(&String::from("value_c")));

        let my_value = cache.get(&"X");
        assert_eq!(my_value, None);

        cache.put("A", String::from("value_a"));
        cache.put("X", String::from("value_x"));

        let my_value = cache.get(&"B");
        assert_eq!(my_value, None);

        let my_value = cache.get(&"D");
        assert_eq!(my_value, None);
    }
}