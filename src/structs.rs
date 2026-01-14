use std::collections::{HashMap, VecDeque};

/// Structure interne du cache LRU
/// Cette struct contient seulement les données
pub struct Cache<K, V> where K: std::fmt::Display{
    pub(crate) size: usize, // capacité maximale du cache

    /// Associe une clé à sa valeur et son compteur d'accès
    pub(crate) map: HashMap<K, (V, u64)>, // accès O(1) avec compteur

    /// Ordre d'insertion pour l'éviction LRU
    pub(crate) ordre: VecDeque <K>, // pour l'éviction

    /// Compteur global d'accès
    pub(crate) access_counter: u64,
}