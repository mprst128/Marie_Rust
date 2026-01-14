//! # Crate `lru_cache`
//!
//! Ce crate fournit une implémentation simple et générique d’un cache LRU,
//! basé sur :
//!
//! - une structure interne utilisant **HashMap + VecDeque**
//! - un trait générique `LruCache` pour abstraction
//! - une version persistante capable de sauvegarder/charger depuis un fichier
//! - une architecture modulaire et claire
//!
//! # Exemple d'utilisation
//! ```rust
//! use lru_cache::{Cache, LruCache};
//!
//! let mut cache = Cache::new(2);
//! cache.put("A".to_string(), 1);
//! cache.put("B".to_string(), 2);
//! assert_eq!(cache.get(&"A".to_string()), Some(&1));
//!
//! cache.put("C".to_string(), 3); // évince B
//! assert_eq!(cache.get(&"B".to_string()), None);
//! ```

/// Module contenant l’implémentation du cache LRU
pub mod cache;

/// Module définissant les erreurs (`CacheError`) et le type résultat (`CacheResult`)
pub mod errors;

/// Module contenant le trait `LruCache`
pub mod traits;

/// Module contenant la structure interne `Cache`
pub mod structs;

/// Module ajoutant la persistance (lecture/écriture dans un fichier texte)
pub mod persistent;

// -----------------------------------------------------------------------------
// Réexportations publiques
// -----------------------------------------------------------------------------

/// Réexportation de la structure principale `Cache`.
pub use structs::Cache;

/// Réexportation des types d’erreurs liés au cache.
pub use errors::{CacheError, CacheResult};

/// Réexportation du trait `LruCache`.
pub use traits::LruCache;