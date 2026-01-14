use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

use crate::errors::{CacheError, CacheResult};
use crate::structs::Cache;
use crate::traits::LruCache;

/// Extension de persistance pour le cache LRU
/// Permet de charger un cache depuis un fichier et de le sauvegarder
///
/// Format du fichier :
/// ```text
/// clé=valeur
/// clé=valeur
/// ```
///
/// # Exemple d'utilisation
/// ```rust
/// use lru_cache::{Cache, LruCache};
/// use std::fs;
///
/// let cache_path = "test_cache.txt";
/// 
/// // Créer un cache persistant
/// let mut cache = Cache::<String, String>::new_persistent(2, cache_path);
/// cache.put("A".to_string(), "1".to_string());
/// cache.save(cache_path).unwrap();
///
/// // Charger depuis le fichier
/// let loaded = Cache::<String, String>::new_persistent(2, cache_path);
/// assert_eq!(loaded.len(), 1);
/// 
/// // Nettoyer
/// let _ = fs::remove_file(cache_path);
/// ```
///
/// # Contraintes
/// - K: ToString + FromStr
/// - V: ToString + FromStr
impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone + ToString + FromStr,
    V: Clone + ToString + FromStr,
{
    /// Charge un cache depuis un fichier
    /// Si le fichier n'existe pas, crée un cache vide
    ///
    /// # Exemple d'utilisation
    /// ```rust
    /// use lru_cache::{Cache, LruCache};
    /// use std::fs;
    /// 
    /// let cache = Cache::<String, String>::new_persistent(3, "nonexistent.txt");
    /// assert_eq!(cache.capacity(), 3);
    /// assert_eq!(cache.len(), 0);
    /// ```
    pub fn new_persistent(size: usize, path: &str) -> Self {
        let mut cache = Cache::new(size);

        // Si le fichier existe, on le lit
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);

            for line in reader.lines().flatten() {
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
        }

        cache
    }

    /// Sauvegarde le contenu du cache dans un fichier
    pub fn save(&self, path: &str) -> CacheResult<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(CacheError::Io)?;

        // On parcourt toutes les clés du cache
        for (key, &index) in &self.map {
            if let Some(Some(value)) = self.values.get(index) {
                let line = format!("{}={}\n", key.to_string(), value.to_string());
                file.write_all(line.as_bytes()).map_err(CacheError::Io)?;
            }
        }

        Ok(())
    }
}