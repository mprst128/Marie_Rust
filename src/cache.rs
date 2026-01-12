use std::collections::HashMap;
use std::hash::Hash;

/// Un nœud de la liste doublement chaînée
#[derive(Debug, Clone)]
struct Node {
    prev: Option<usize>, // index du nœud précédent dans la liste
    next: Option<usize>, // index du nœud suivant dans la liste
}

/// Cache LRU en O(1)
pub struct Cache<K, V> {
    size: usize, // capacité maximale du cache

    /// Liste doublement chaînée représentée par un tableau
    nodes: Vec<Option<Node>>,  // stocke les liens prev/next pour chaque entrée
    values: Vec<Option<V>>,    // stocke les valeurs
    keys: Vec<Option<K>>,      // stocke les clés

    /// Pointeurs vers la tête (LRU) et la queue (MRU)
    head: Option<usize>, // index du nœud le moins récemment utilisé
    tail: Option<usize>, // index du nœud le plus récemment utilisé

    /// Associe une clé à un index dans `nodes`
    map: HashMap<K, usize>, // permet un accès O(1) à l’index d’un nœud

    /// Pile d’indices libres pour réutiliser les cases
    free_list: Vec<usize>, // indices disponibles pour de nouvelles insertions
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone, // la clé doit être comparable, hashable et clonable
{
    /// Crée un nouveau cache LRU
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "La capacité doit être > 0"); // sécurité

        Self {
            size,
            nodes: vec![None; size],  // tableau vide de nœuds
            values: {
                let mut v = Vec::with_capacity(size);
                v.resize_with(size, || None);
                v
            }, // tableau vide de valeurs
            keys: vec![None; size],   // tableau vide de clés
            head: None,                   // pas encore de LRU
            tail: None,                   // pas encore de MRU
            map: HashMap::with_capacity(size), // accès rapide clé → index
            free_list: (0..size).rev().collect(), // indices libres empilés
        }
    }

    /// Récupère une valeur en O(1)
    pub fn get(&mut self, key: &K) -> Option<&V> {
        // On cherche l’index associé à la clé
        if let Some(&index) = self.map.get(key) {
            // On marque l’entrée comme récemment utilisée
            self.move_to_tail(index);
            // On retourne la valeur stockée
            self.values[index].as_ref()
        } else {
            None // clé absente du cache
        }
    }

    /// Ajoute ou met à jour une entrée en O(1)
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
    // Si la clé existe déjà → mise à jour + move_to_tail
        if let Some(&index) = self.map.get(&key) {
            let old_value = self.values[index].take(); // récupérer l’ancienne valeur
            self.values[index] = Some(value);          // mettre la nouvelle
            self.move_to_tail(index);                  // devient MRU
            return old_value;                          // renvoyer l’ancienne valeur
        }

        // Si plein → éviction du LRU
        let mut evicted_value = None;
        if self.map.len() == self.size {
         evicted_value = self.evict_lru(); // récupère Some(V) ou None
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

        // Mettre à jour la liste chaînée
        if let Some(tail_index) = self.tail {
            if let Some(Some(tail_node)) = self.nodes.get_mut(tail_index) {
                tail_node.next = Some(index);
            }
        }
        if self.head.is_none() {
            self.head = Some(index);
        }

        self.tail = Some(index);
        self.map.insert(key, index);
        evicted_value
    }   

    /// Évite le LRU (head)
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
            } 
            else {
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

    /// Déplace un nœud vers la queue (MRU) en O(1)
    fn move_to_tail(&mut self, index: usize) {
        // Si déjà MRU → rien à faire
        if Some(index) == self.tail {
            return;
        }

        // Récupérer les pointeurs prev/next du nœud
        let (prev, next) = if let Some(Some(node)) = self.nodes.get(index) {
            (node.prev, node.next)
        } else {
            return; // nœud invalide
        };

        // Retirer le nœud de sa position actuelle
        if let Some(prev_index) = prev {
            // Le précédent saute ce nœud
            if let Some(Some(prev_node)) = self.nodes.get_mut(prev_index) {
                prev_node.next = next;
            }
        } else {
            // Le nœud était le head → on avance le head
            self.head = next;
        }

        if let Some(next_index) = next {
            // Le suivant saute ce nœud
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

        // Mettre à jour les pointeurs du nœud déplacé
        if let Some(Some(node)) = self.nodes.get_mut(index) {
            node.prev = self.tail; // son précédent devient l’ancienne queue
            node.next = None;      // il devient la nouvelle queue
        }

        // Mettre à jour le pointeur global tail
        self.tail = Some(index);
    }

    /// Nombre d’éléments
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Capacité max
    pub fn size(&self) -> usize {
        self.size
    }

    /// Vide ?
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}