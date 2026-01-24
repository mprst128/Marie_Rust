use std::collections::{HashMap, VecDeque};
use std::fs::{read_to_string, write};
use std::hash::Hash;
use std::str::FromStr;

use crate::structs::Cache;
use crate::traits::LruCache;

/// Implémentation des méthodes de base du cache LRU
impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Display + ToString + FromStr, //Eq et Hash pour HashMap, Clone pour manipuler les clés, Display et ToString pour la persistance, FromStr pour le chargement
    V: Clone + ToString + FromStr,
{
    /// Crée un nouveau cache LRU avec une capacité donnée
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "La capacité doit être > 0");

        Self {
            size,
            map: HashMap::new(), // allocation réduite si on ajoute mémoire donc mettre capacity à l'intérieur
            ordre: VecDeque::new(),
            access_counter: 0,
        }
    }

    /// Ajoute ou met à jour une entrée en O(1)
    ///
    /// - Si la clé existe déjà : met à jour la valeur et retourne l’ancienne
    /// - Si la clé n’existe pas :
    ///   - si le cache est plein → éviction du LRU
    ///   - insertion de la nouvelle entrée
    ///
    /// # Exemple d'utilisation
    /// ```rust
    /// use lru_cache::{Cache, LruCache};
    ///
    /// let mut cache = Cache::new(2);
    ///
    /// assert_eq!(cache.put("A".to_string(), 1), None);
    /// assert_eq!(cache.put("A".to_string(), 2), Some(1)); // mise à jour
    /// ```
    
    
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        // Si la clé existe déjà, on met à jour la valeur et le compteur d'accès
        if self.map.contains_key(&key) {
            self.access_counter += 1;
            let old = self.map.insert(key.clone(), (value, self.access_counter));
            return old.map(|(v, _)| v);
        }

        // Si la clé n'existe pas encore : si plein → éviction du LRU
        if self.map.len() == self.size {
            self.go_back();
        }

        // On insère la nouvelle entrée
        self.access_counter += 1;
        self.map.insert(key.clone(), (value, self.access_counter));
        self.ordre.push_back(key);

        None
    }

    /// Récupère une valeur et met à jour l'ordre LRU - O(1)
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some((value, access_time)) = self.map.get_mut(key) {
            self.access_counter += 1;
            *access_time = self.access_counter; // Met à jour le compteur d'accès
            Some(value)
        } else {
            None
        }
    }

    /// Nombre d’éléments actuellement dans le cache
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Capacité maximale du cache
    pub fn capacity(&self) -> usize {
        self.size
    }

    /// Éviction de l'élément le moins récemment utilisé
    fn go_back(&mut self) {
        // Trouve la clé avec le plus petit compteur d'accès
        if let Some((&ref lru_key, _)) = self.map.iter()
            .min_by_key(|(_, (_, access_time))| *access_time) {
            let lru_key = lru_key.clone();
            self.map.remove(&lru_key);
            // Supprime aussi de la VecDeque
            if let Some(pos) = self.ordre.iter().position(|k| k == &lru_key) {
                self.ordre.remove(pos);
            }
        }
    }

    //chaque acces à get et put incrémente le compteur global d'accès et met à jour le compteur de l'élément accédé
    //l'élément le plus petit compteur est le LRU
    //complexité 0(n) donc pas optimal 

    /// Sauvegarde le cache dans un fichier sous forme "clé=valeur"
    pub fn save_cache(&self, path: &str) {
        let mut lines: Vec<String> = Vec::new();

        // On sauvegarde dans l'ordre LRU → MRU (ordre des compteurs d'accès)
        let mut entries: Vec<(&K, &(V, u64))> = self.map.iter().collect();
        entries.sort_by_key(|(_, (_, access_time))| *access_time);
        
        for (key, (value, _)) in entries {
            lines.push(format!("{}={}", key.to_string(), value.to_string()));
        }

        let content = lines.join("\n");
        let _ = write(path, content);
    }

    /// Charge un cache depuis un fichier texte
    pub fn load_cache(&mut self, path: &str) {
        if let Ok(content) = read_to_string(path) {
            for line in content.lines() {
                if let Some((k_str, v_str)) = line.split_once('=') {
                    if let (Ok(k), Ok(v)) = (K::from_str(k_str), V::from_str(v_str)) {
                        self.put(k, v);
                    }
                }
            }
        }
    }
}


//

/// Implémentation du trait LruCache en se basant sur les méthodes ci-dessus
impl<K, V> LruCache<K, V> for Cache<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Display + ToString + FromStr,
    V: Clone + ToString + FromStr,
{
    fn get(&mut self, key: &K) -> Option<&V> {
        Cache::get(self, key)
    }

    fn put(&mut self, key: K, value: V) -> Option<V> {
        Cache::put(self, key, value)
    }

    fn len(&self) -> usize {
        Cache::len(self)
    }

    fn capacity(&self) -> usize {
        Cache::capacity(self)
    }

    fn save_cache(&self, path: &str) {
        Cache::save_cache(self, path)
    }

    fn load_cache(&mut self, path: &str) {
        Cache::load_cache(self, path)
    }
}