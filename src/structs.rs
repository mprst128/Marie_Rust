use std::collections::HashMap;

/// Un nœud de la liste doublement chaînée.
/// Il représente un élément du cache dans la structure interne.
///
/// # Exemple
/// ```rust
/// use lru_cache::Node;
/// 
/// let node = Node { prev: Some(0), next: Some(2) };
/// assert_eq!(node.prev, Some(0));
/// assert_eq!(node.next, Some(2));
/// ```
#[derive(Debug, Clone)]
pub struct Node {
    pub prev: Option<usize>, // index du nœud précédent
    pub next: Option<usize>, // index du nœud suivant
}

/// Structure interne du cache LRU.
/// Cette struct ne contient **aucune logique**, seulement les données.
pub struct Cache<K, V> {
    pub size: usize, // capacité maximale du cache

    /// Liste doublement chaînée représentée par un tableau
    pub nodes: Vec<Option<Node>>,  // stocke les liens prev/next
    pub values: Vec<Option<V>>,    // stocke les valeurs
    pub keys: Vec<Option<K>>,      // stocke les clés

    /// Pointeurs vers la tête (LRU) et la queue (MRU)
    pub head: Option<usize>, // index du nœud le moins récemment utilisé
    pub tail: Option<usize>, // index du nœud le plus récemment utilisé

    /// Associe une clé à un index dans `nodes`
    pub map: HashMap<K, usize>, // accès O(1) à l’index d’un nœud

    /// Pile d’indices libres pour réutiliser les cases
    pub free_list: Vec<usize>, // indices disponibles pour de nouvelles insertions
}