use lru_cache::{Cache, LruCache};
use std::fs;


//
// ─────────────────────────────────────────────────────────────
//   TESTS PROF
// ─────────────────────────────────────────────────────────────
//


#[test]
fn test_lru_cache_prof() {
    let mut cache = Cache::new(3); // Taille de 3

    cache.put("A".to_string(), String::from("Marie"));
    cache.put("B".to_string(), String::from("Léane"));
    cache.put("C".to_string(), String::from("Maia"));
    cache.put("D".to_string(), String::from("Nicolas"));
    // Premier élément moins récemment utilisé et dernier le plus récent
    // Cache == [B, C, D]

    let my_value = cache.get(&"A".to_string());
    assert_eq!(my_value, None);

    let my_value = cache.get(&"D".to_string());
    assert_eq!(my_value, Some(&String::from("Nicolas")));
    // Cache == [B, C, D]

    let my_value = cache.get(&"B".to_string());
    assert_eq!(my_value, Some(&String::from("Léane")));
    // Cache == [C, D, B]

    let my_value = cache.get(&"C".to_string());
    assert_eq!(my_value, Some(&String::from("Maia")));
    // Cache == [D, B, C]

    let my_value = cache.get(&"X".to_string());
    assert_eq!(my_value, None);
    // Cache == [D, B, C]

    cache.put("A".to_string(), String::from("Marie"));
    // Cache == [B, C, A]

    cache.put("X".to_string(), String::from("Noa"));
    // Cache == [C, A, X]

    let my_value = cache.get(&"B".to_string());
    assert_eq!(my_value, None);
    // Cache == [C, A, X]

    let my_value = cache.get(&"D".to_string());
    assert_eq!(my_value, None);
    // Cache == [C, A, X]
}


//
// ─────────────────────────────────────────────────────────────
//   TESTS POUR cache.rs
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que `new` crée un cache vide avec la bonne capacité
#[test]
fn test_new_creates_empty_cache() {
    let cache = Cache::<i32, i32>::new(3);
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.capacity(), 3);
}

/// Vérifie que `new(0)` renvoi bien une erreur comme prévu
#[test]
fn test_new_zero_capacity() {
    let result = std::panic::catch_unwind(|| Cache::<i32, i32>::new(0));
    assert!(result.is_err());
}

/// Vérifie que `get` retourne une valeur existante
#[test]
fn test_get_returns_value() {
    let mut cache = Cache::new(2);
    cache.put("A".to_string(), 10);
    assert_eq!(cache.get(&"A".to_string()), Some(&10));
}

/// Vérifie que `get` retourne None si la clé n’existe pas
#[test]
fn test_get_returns_none_for_missing_key() {
    let mut cache = Cache::<String, i32>::new(2);
    assert_eq!(cache.get(&"X".to_string()), None);
}

/// Vérifie que `put` insère une nouvelle entrée
#[test]
fn test_put_inserts_new_value() {
    let mut cache = Cache::new(2);
    assert_eq!(cache.put("A".to_string(), 1), None);
    assert_eq!(cache.get(&"A".to_string()), Some(&1));
}

/// Vérifie que `put` met à jour une entrée existante
#[test]
fn test_put_updates_existing_value() {
    let mut cache = Cache::new(2);
    cache.put("A".to_string(), 1);
    assert_eq!(cache.put("A".to_string(), 2), Some(1));
    assert_eq!(cache.get(&"A".to_string()), Some(&2));
}

/// Vérifie que `put` déclenche une éviction quand le cache est plein
#[test]
fn test_put_eviction_occurs() {
    let mut cache = Cache::new(2);
    cache.put("A".to_string(), 1);
    cache.put("B".to_string(), 2);
    cache.put("C".to_string(), 3); // évince A
    assert_eq!(cache.get(&"A".to_string()), None);
}

/// Vérifie que `get` déplace la clé en MRU
#[test]
fn test_get_moves_key_to_mru() {
    let mut cache = Cache::new(2);
    cache.put("A".to_string(), 1);
    cache.put("B".to_string(), 2);
    cache.get(&"A".to_string()); // A devient MRU
    cache.put("C".to_string(), 3); // évince B
    assert_eq!(cache.get(&"B".to_string()), None);
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS POUR persistent.rs
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que la persistance fonctionne avec save et load
#[test]
fn test_persistent_save_load() {
    use lru_cache::persistent::persistent;
    let path = "test_persistent_load.txt";
    
    let mut cache = Cache::<String, String>::new(3);
    cache.put("A".to_string(), "Marie".to_string());
    cache.put("B".to_string(), "Léane".to_string());
    
    // Sauvegarde
    persistent::save(path, &cache).unwrap();
    
    // Nouveau cache pour charger
    let mut new_cache = Cache::<String, String>::new(3);
    persistent::load(path, &mut new_cache).unwrap();

    assert_eq!(new_cache.get(&"A".to_string()), Some(&"Marie".to_string()));
    assert_eq!(new_cache.get(&"B".to_string()), Some(&"Léane".to_string()));

    let _ = fs::remove_file(path);
}

/// Vérifie que le cache fonctionne sans fichier de persistance
#[test]
fn test_cache_without_persistence() {
    let cache = Cache::<String, String>::new(3);
    assert_eq!(cache.len(), 0);
}

/// Vérifie que `save` crée bien un fichier
#[test]
fn test_save_creates_file() {
    use lru_cache::persistent::persistent;
    let path = "test_save_file.txt";
    let _ = fs::remove_file(path);

    let mut cache = Cache::<String, String>::new(2);
    cache.put("A".to_string(), "Marie".to_string());
    persistent::save(path, &cache).unwrap();

    assert!(fs::metadata(path).is_ok());
    let _ = fs::remove_file(path);
}

/// Vérifie que `save` écrit correctement les paires clé=valeur
#[test]
fn test_save_writes_correct_content() {
    use lru_cache::persistent::persistent;
    let path = "test_save_content.txt";
    let _ = fs::remove_file(path);

    let mut cache = Cache::<String, String>::new(2);
    cache.put("A".to_string(), "Marie".to_string());
    cache.put("B".to_string(), "Léane".to_string());
    persistent::save(path, &cache).unwrap();

    let content = fs::read_to_string(path).unwrap();
    assert!(content.contains("A=Marie"));
    assert!(content.contains("B=Léane"));

    let _ = fs::remove_file(path);
}


//
// ─────────────────────────────────────────────────────────────
//   TESTS POUR traits.rs
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que `get` du trait appelle bien l’implémentation interne
#[test]
fn test_trait_get_works() {
    let mut cache = Cache::new(2);
    cache.put("A".to_string(), 1);
    assert_eq!(LruCache::get(&mut cache, &"A".to_string()), Some(&1));
}

/// Vérifie que `put` du trait appelle bien l’implémentation interne
#[test]
fn test_trait_put_works() {
    let mut cache = Cache::new(2);
    assert_eq!(LruCache::put(&mut cache, "A".to_string(), 1), None);
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS POUR structs.rs
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que la structure interne démarre vide
#[test]
fn test_structs_initial_state() {
    let cache = Cache::<i32, i32>::new(3);
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.capacity(), 3);
}

/// Vérifie que la structure interne a la bonne capacité
#[test]
fn test_structs_capacity() {
    let cache = Cache::<i32, i32>::new(3);
    assert_eq!(cache.capacity(), 3);
}