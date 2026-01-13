//! Implémentation du cache LRU (Least Recently Used) en O(1).
//!
//! Ce module contient l’implémentation principale du cache LRU,
//! basée sur :
//! - un tableau de nœuds représentant une liste doublement chaînée,
//! - un `HashMap` pour retrouver les indices en O(1),
//! - une pile d’indices libres pour réutiliser les cases,
//! - une gestion explicite du LRU (head) et du MRU (tail).
//!
//! # Exemple simple
//! ```rust
//! use lru_cache::{Cache, LruCache};
//!
//! let mut cache = Cache::new(2);
//! cache.put("A", 1);
//! cache.put("B", 2);
//! assert_eq!(cache.get(&"A"), Some(&1)); // A devient MRU
//!
//! cache.put("C", 3); // évince B (LRU)
//! assert_eq!(cache.get(&"B"), None);
//! ```

use std::hash::Hash;
use crate::structs::{Cache, Node};
use crate::traits::LruCache;

//
// ─────────────────────────────────────────────────────────────
//   IMPLÉMENTATION DU CACHE LRU O(1)
// ─────────────────────────────────────────────────────────────
//

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Crée un nouveau cache LRU avec une capacité fixe.
    ///
    /// # Panics
    /// Panique si `size == 0`.
    ///
    /// # Exemple
    /// ```rust
    /// use lru_cache::Cache;
    /// use lru_cache::LruCache;
    /// let cache = Cache::<i32, i32>::new(3);
    /// assert_eq!(cache.capacity(), 3);
    /// ```
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "La capacité doit être > 0");

        Self {
            size,
            nodes: vec![None; size],
            values: {
                let mut v = Vec::with_capacity(size);
                v.resize_with(size, || None);
                v
            },
            keys: vec![None; size],
            head: None,
            tail: None,
            map: std::collections::HashMap::with_capacity(size),
            free_list: (0..size).rev().collect(),
        }
    }

    /// Récupère une valeur en O(1) et marque l’entrée comme MRU.
    ///
    /// Si la clé existe :
    /// - elle est déplacée en queue (MRU),
    /// - la valeur est retournée.
    ///
    /// Sinon, retourne `None`.
    ///
    /// # Exemple
    /// ```rust
    /// use lru_cache::{Cache, LruCache};
    /// let mut cache = Cache::new(2);
    /// cache.put("A", 10);
    /// assert_eq!(cache.get(&"A"), Some(&10));
    /// ```
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(&index) = self.map.get(key) {
            self.move_to_tail(index);
            self.values[index].as_ref()
        } else {
            None
        }
    }

    /// Ajoute ou met à jour une entrée en O(1).
    ///
    /// - Si la clé existe déjà : met à jour la valeur et retourne l’ancienne.
    /// - Si la clé n’existe pas :
    ///   - si le cache est plein → éviction du LRU,
    ///   - insertion de la nouvelle entrée.
    ///
    /// # Exemple
    /// ```rust
    /// use lru_cache::{Cache, LruCache};
    /// let mut cache = Cache::new(2);
    ///
    /// assert_eq!(cache.put("A", 1), None);
    /// assert_eq!(cache.put("A", 2), Some(1)); // mise à jour
    /// ```
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        // Mise à jour si la clé existe déjà
        if let Some(&index) = self.map.get(&key) {
            let old_value = self.values[index].take();
            self.values[index] = Some(value);
            self.move_to_tail(index);
            return old_value;
        }

        // Cache plein → éviction du LRU
        let mut evicted_value = None;
        if self.map.len() == self.size {
            evicted_value = self.evict_lru();
        }

        // Récupérer un index libre
        let index = self.free_list.pop().expect("free_list vide");

        // Insérer la nouvelle entrée
        self.keys[index] = Some(key.clone());
        self.values[index] = Some(value);
        self.nodes[index] = Some(Node {
            prev: self.tail,
            next: None,
        });

        // Mettre à jour l’ancienne queue
        if let Some(tail_index) = self.tail {
            if let Some(Some(tail_node)) = self.nodes.get_mut(tail_index) {
                tail_node.next = Some(index);
            }
        }

        // Si la liste était vide
        if self.head.is_none() {
            self.head = Some(index);
        }

        self.tail = Some(index);
        self.map.insert(key, index);

        evicted_value
    }

    /// Évite le LRU (head) et retourne la valeur évincée.
    ///
    /// Cette fonction :
    /// - retire la clé du `HashMap`,
    /// - met à jour les pointeurs de la liste doublement chaînée,
    /// - libère l’emplacement dans les tableaux internes.
    ///
    /// Retourne `Some(V)` si une valeur a été évincée, sinon `None`.
    fn evict_lru(&mut self) -> Option<V> {
        if let Some(lru_index) = self.head {
            // Retirer la clé du HashMap
            if let Some(Some(k)) = self.keys.get(lru_index) {
                self.map.remove(k);
            }

            // Récupérer la valeur évincée
            let evicted_value = self.values[lru_index].take();

            // Mettre à jour la liste
            let next = self.nodes[lru_index].as_ref().unwrap().next;
            self.head = next;

            if let Some(next_index) = next {
                if let Some(Some(next_node)) = self.nodes.get_mut(next_index) {
                    next_node.prev = None;
                }
            } else {
                self.tail = None;
            }

            // Libérer l’emplacement
            self.nodes[lru_index] = None;
            self.keys[lru_index] = None;
            self.free_list.push(lru_index);

            return evicted_value;
        }
        None
    }

    /// Déplace un nœud vers la queue (MRU) en O(1).
    ///
    /// Cette opération est centrale dans un cache LRU :
    /// - un accès (`get`) ou une mise à jour (`put`) rend l’entrée MRU,
    /// - la queue représente l’élément le plus récemment utilisé.
    fn move_to_tail(&mut self, index: usize) {
        // Déjà MRU → rien à faire
        if Some(index) == self.tail {
            return;
        }

        // Récupérer les pointeurs du nœud
        let (prev, next) = if let Some(Some(node)) = self.nodes.get(index) {
            (node.prev, node.next)
        } else {
            return;
        };

        // Retirer le nœud de sa position actuelle
        if let Some(prev_index) = prev {
            if let Some(Some(prev_node)) = self.nodes.get_mut(prev_index) {
                prev_node.next = next;
            }
        } else {
            self.head = next;
        }

        if let Some(next_index) = next {
            if let Some(Some(next_node)) = self.nodes.get_mut(next_index) {
                next_node.prev = prev;
            }
        }

        // Ajouter en queue
        if let Some(tail_index) = self.tail {
            if let Some(Some(tail_node)) = self.nodes.get_mut(tail_index) {
                tail_node.next = Some(index);
            }
        }

        if let Some(Some(node)) = self.nodes.get_mut(index) {
            node.prev = self.tail;
            node.next = None;
        }

        self.tail = Some(index);
    }
}

//
// ─────────────────────────────────────────────────────────────
//   IMPLÉMENTATION DU TRAIT LruCache
// ─────────────────────────────────────────────────────────────
//

impl<K, V> LruCache<K, V> for Cache<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Appelle [`Cache::get`] pour récupérer une valeur.
    fn get(&mut self, key: &K) -> Option<&V> {
        Cache::get(self, key)
    }

    /// Appelle [`Cache::put`] pour insérer ou mettre à jour une valeur.
    fn put(&mut self, key: K, value: V) -> Option<V> {
        Cache::put(self, key, value)
    }

    /// Retourne le nombre d’éléments actuellement stockés.
    fn len(&self) -> usize {
        self.map.len()
    }

    /// Retourne la capacité maximale du cache.
    fn capacity(&self) -> usize {
        self.size
    }
}