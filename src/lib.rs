//! # Crate `lru_cache`
//!
//! Ce crate fournit une implémentation complète d’un cache LRU
//! entièrement en Rust, avec :
//!
//! - une implémentation **O(1)** basée sur une liste doublement chaînée + HashMap
//! - un **trait générique** `LruCache` pour abstraction
//! - une version **persistante** capable de sauvegarder/charger depuis un fichier
//! - une gestion d’erreurs propre via `CacheError` et `CacheResult`
//! - une architecture modulaire et claire
//!
//! # Exemple d'utilisation
//! ```rust
//! use lru_cache::{Cache, LruCache};
//!
//! let mut cache = Cache::new(2);
//! cache.put("A", 1);
//! cache.put("B", 2);
//! assert_eq!(cache.get(&"A"), Some(&1));
//!
//! cache.put("C", 3); // évince B
//! assert_eq!(cache.get(&"B"), None);
//! ```

/// Module contenant l’implémentation du cache LRU en O(1)
pub mod cache;

/// Module définissant les erreurs (`CacheError`) et le type résultat (`CacheResult`)
pub mod errors;

/// Module contenant le trait `LruCache`, permettant d’abstraire l’implémentation
pub mod traits;

/// Module contenant les structures internes (`Cache`, `Node`)
pub mod structs;

/// Module ajoutant la persistance (lecture/écriture dans un fichier texte)
pub mod persistent;

// -----------------------------------------------------------------------------
// Réexportations publiques
// -----------------------------------------------------------------------------
// Ces réexportations permettent d'utiliser le crate plus facilement :
// `use lru_cache::Cache;` au lieu de `use lru_cache::structs::Cache;`.

/// Réexportation de la structure principale `Cache`.
pub use structs::Cache;
/// Réexportation de la structure `Node` pour les tests.
pub use structs::Node;
/// Réexportation des types d’erreurs liés au cache.
pub use errors::{CacheError, CacheResult};

/// Réexportation du trait `LruCache`.
pub use traits::LruCache;