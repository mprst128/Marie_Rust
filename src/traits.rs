//! Trait définissant l’interface d’un cache LRU
//!
//! Il permet d'utiliser différentes implémentations via une API commune
//!
//! # Exemple d'utilisation
//! ```rust
//! use lru_cache::{Cache, LruCache};
//!
//! fn test_cache<T: LruCache<String, i32>>(mut cache: T) {
//!     assert_eq!(cache.put("A".to_string(), 1), None);
//!     assert_eq!(cache.get(&"A".to_string()), Some(&1));
//!     assert_eq!(cache.len(), 1);
//! }
//!
//! let cache = Cache::new(3);
//! test_cache(cache); // Cache implémente LruCache
//! ```
/// Interface minimale d’un cache LRU
///
/// - `get` : récupère une valeur et la marque comme MRU  
/// - `put` : insère ou met à jour une entrée  
/// - `len` : nombre d’éléments stockés  
/// - `capacity` : capacité maximale du cache
///
/// # Exemple d'utilisation
/// ```rust
/// use lru_cache::{Cache, LruCache};
///
/// let mut cache = Cache::new(2);
/// 
/// // Utilisation via le trait
/// cache.put("key1", 100);
/// cache.put("key2", 200);
/// assert_eq!(cache.len(), 2);
/// assert_eq!(cache.capacity(), 2);
/// 
/// assert_eq!(cache.get(&"key1"), Some(&100));
/// assert_eq!(cache.get(&"nonexistent"), None);
/// ```
pub trait LruCache<K, V> {
    /// Retourne la valeur associée à `key`, ou `None` si absente
    fn get(&mut self, key: &K) -> Option<&V>;

    /// Ajoute ou met à jour une entrée
    /// Retourne l’ancienne valeur si la clé existait
    fn put(&mut self, key: K, value: V) -> Option<V>;

    /// Nombre d’éléments actuellement dans le cache
    fn len(&self) -> usize;

    /// Capacité maximale du cache
    fn capacity(&self) -> usize;
}