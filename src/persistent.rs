use std::fs::{OpenOptions, read_to_string};
use std::io::{Write};
use std::str::FromStr;

use crate::errors::{CacheError, CacheResult};
use crate::structs::Cache;

/// Extension de persistance pour le cache LRU.
/// Permet de charger un cache depuis un fichier et de le sauvegarder.
///
/// Format du fichier :
/// ```text
/// clé=valeur
/// clé=valeur
///
/// # Contraintes
/// - K: ToString + FromStr
/// - V: ToString + FromStr

/// Fonctions de persistance accessibles via `persistent::save()` et `persistent::load()`
pub mod persistent {
    use super::*;

/// # Exemple d'utilisation
/// 
/// ```rust
/// use lru_cache::structs::Cache;
/// use lru_cache::persistent::persistent;
/// use lru_cache::traits::LruCache;
///
/// # fn main() {
/// let mut cache: Cache<String, String> = Cache::new(3);
/// cache.put("A".to_string(), "value_a".to_string());
/// cache.put("B".to_string(), "value_b".to_string());
///
/// persistent::save("mon_cache.txt", &cache).unwrap();
/// # let _ = std::fs::remove_file("mon_cache.txt");
/// # }
/// ```

    /// Sauvegarde un cache dans un fichier texte.
    pub fn save<K, V>(path: &str, cache: &Cache<K, V>) -> CacheResult<()>
    where
        K: Eq + std::hash::Hash + Clone + std::fmt::Display + ToString,
        V: Clone + ToString,
    {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(CacheError::Io)?;

        for key in &cache.ordre {
            if let Some((value, _)) = cache.map.get(key) {
                let line = format!("{}={}\n", key.to_string(), value.to_string());
                file.write_all(line.as_bytes()).map_err(CacheError::Io)?;
            }
        }

        Ok(())
    }


/// # Exemple d'utilisation
/// 
/// ```rust
/// use lru_cache::structs::Cache;
/// use lru_cache::persistent::persistent;
///
/// # let mut cache: Cache<String, String> = Cache::new(3);
/// # cache.put("A".to_string(), "value_a".to_string());
/// # persistent::save("mon_cache.txt", &cache).unwrap();
/// let mut cache: Cache<String, String> = Cache::new(3);
/// persistent::load("mon_cache.txt", &mut cache).unwrap();
///
/// let _ = std::fs::remove_file("mon_cache.txt");
/// ```

    /// Charge un cache depuis un fichier texte.
    pub fn load<K, V>(path: &str, cache: &mut Cache<K, V>) -> CacheResult<()>
    where
        K: Eq + std::hash::Hash + Clone + std::fmt::Display + ToString + FromStr,
        V: Clone + ToString + FromStr,
    {
        let content = read_to_string(path).map_err(CacheError::Io)?;

        for line in content.lines() {
            if let Some((k_str, v_str)) = line.split_once('=') {
                let key = K::from_str(k_str);
                let value = V::from_str(v_str);

                match (key, value) {
                    (Ok(k), Ok(v)) => {
                        cache.put(k, v);
                    }
                    (Err(_), _) => {
                        eprintln!("Erreur de parsing clé : {k_str:?}");
                    }
                    (_, Err(_)) => {
                        eprintln!("Erreur de parsing valeur : {v_str:?}");
                    }
                }
            }
        }

        Ok(())
    }
}
