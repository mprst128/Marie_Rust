use lru_cache::{Cache};
use std::fs;

//
// ─────────────────────────────────────────────────────────────
//   TESTS D'INTÉGRATION
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que le cache insère correctement plusieurs valeurs
/// et que les accès fonctionnent comme prévu
#[test]
fn test_lru_basic_insertion_and_access() {
    let mut cache = Cache::new(3);

    cache.put("A".to_string(), "Marie".to_string());
    cache.put("B".to_string(), "Léane".to_string());
    cache.put("C".to_string(), "Maia".to_string());

    assert_eq!(cache.get(&"A".to_string()), Some(&"Marie".to_string()));
    assert_eq!(cache.get(&"B".to_string()), Some(&"Léane".to_string()));
    assert_eq!(cache.get(&"C".to_string()), Some(&"Maia".to_string()));
}

/// Vérifie que l'éviction LRU fonctionne sur un scénario
#[test]
fn test_lru_eviction_simple() {
    let mut cache = Cache::new(2);

    cache.put("A".to_string(), 1);
    cache.put("B".to_string(), 2);
    cache.put("C".to_string(), 3); // évince A

    assert_eq!(cache.get(&"A".to_string()), None);
    assert_eq!(cache.get(&"B".to_string()), Some(&2));
    assert_eq!(cache.get(&"C".to_string()), Some(&3));
}

/// Vérifie que l'accès à une clé la rend MRU
/// et influence l'ordre d'éviction
#[test]
fn test_lru_access_updates_order() {
    let mut cache = Cache::new(2);

    cache.put("A".to_string(), 1);
    cache.put("B".to_string(), 2);

    cache.get(&"A".to_string()); // A devient MRU

    cache.put("C".to_string(), 3); // évince B

    assert_eq!(cache.get(&"B".to_string()), None);
    assert_eq!(cache.get(&"A".to_string()), Some(&1));
    assert_eq!(cache.get(&"C".to_string()), Some(&3));
}

/// Vérifie que la mise à jour d'une clé existante
/// ne change pas la taille et conserve l'ordre LRU
#[test]
fn test_lru_update_existing_key() {
    let mut cache = Cache::new(2);

    cache.put("A".to_string(), 1);
    cache.put("B".to_string(), 2);

    let old = cache.put("A".to_string(), 99);
    assert_eq!(old, Some(1));

    // A devient MRU
    cache.put("C".to_string(), 3); // évince B

    assert_eq!(cache.get(&"B".to_string()), None);
    assert_eq!(cache.get(&"A".to_string()), Some(&99));
    assert_eq!(cache.get(&"C".to_string()), Some(&3));
}

/// Vérifie que len() et capacity() du trait fonctionnent
#[test]
fn test_trait_len_capacity() {
    let mut cache = Cache::new(2);

    cache.put("A".to_string(), 1);
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.capacity(), 2);
}

/// Test simple de persistance
#[test]
fn test_simple_persistence() {
    use lru_cache::persistent::persistent;
    let path = "test_simple.txt";
    let _ = fs::remove_file(path);

    let mut cache = Cache::<String, String>::new(2);
    cache.put("A".to_string(), "Marie".to_string());
    cache.put("B".to_string(), "Léane".to_string());
    
    // Sauvegarde
    persistent::save(path, &cache).unwrap();
    
    // Nouveau cache pour charger
    let mut new_cache = Cache::<String, String>::new(2);
    persistent::load(path, &mut new_cache).unwrap();
    
    assert_eq!(new_cache.get(&"A".to_string()), Some(&"Marie".to_string()));
    assert_eq!(new_cache.get(&"B".to_string()), Some(&"Léane".to_string()));

    let _ = fs::remove_file(path);
}

/// Vérifie que le cache reste cohérent après plusieurs opérations mixtes
#[test]
fn test_complex_sequence() {
    let mut cache = Cache::new(3);

    cache.put("A".to_string(), 1);
    cache.put("B".to_string(), 2);
    cache.put("C".to_string(), 3);

    cache.get(&"A".to_string()); // A devient MRU
    cache.put("D".to_string(), 4); // évince B

    assert_eq!(cache.get(&"B".to_string()), None);
    assert_eq!(cache.get(&"A".to_string()), Some(&1));
    assert_eq!(cache.get(&"C".to_string()), Some(&3));
    assert_eq!(cache.get(&"D".to_string()), Some(&4));
}