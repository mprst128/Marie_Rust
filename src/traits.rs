//! Trait définissant l’interface d’un cache LRU
/// - `get` : récupère une valeur et la marque comme MRU  
/// - `put` : insère ou met à jour une entrée  
/// - `len` : nombre d’éléments stockés  
/// - `capacity` : capacité maximale du cache
///

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

    /// Sauvegarde le cache dans un fichier sous forme "clé=valeur"
    fn save_cache(&self, path: &str);

    /// Charge un cache depuis un fichier texte
    fn load_cache(&mut self, path: &str);
}