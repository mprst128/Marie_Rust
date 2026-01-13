use lru_cache::{Cache, LruCache, CacheError, CacheResult};
use std::fs;


//
// ─────────────────────────────────────────────────────────────
//   TESTS PROF
// ─────────────────────────────────────────────────────────────
//


#[test]
fn test_lru_cache_prof() {
    let mut cache = Cache::new(3); // Taille de 3

    cache.put("A", String::from("value_a"));
    cache.put("B", String::from("value_b"));
    cache.put("C", String::from("value_c"));
    cache.put("D", String::from("value_d"));
    // Premier élément moins récemment utilisé et dernier le plus récent
    // Cache == [B, C, D]

    let my_value = cache.get(&"A");
    assert_eq!(my_value, None);

    let my_value = cache.get(&"D");
    assert_eq!(my_value, Some(&String::from("value_d")));
    // Cache == [B, C, D]

    let my_value = cache.get(&"B");
    assert_eq!(my_value, Some(&String::from("value_b")));
    // Cache == [C, D, B]

    let my_value = cache.get(&"C");
    assert_eq!(my_value, Some(&String::from("value_c")));
    // Cache == [D, B, C]

    let my_value = cache.get(&"X");
    assert_eq!(my_value, None);
    // Cache == [D, B, C]

    cache.put("A", String::from("value_a"));
    // Cache == [B, C, A]

    cache.put("X", String::from("value_x"));
    // Cache == [C, A, X]

    let my_value = cache.get(&"B");
    assert_eq!(my_value, None);
    // Cache == [C, A, X]

    let my_value = cache.get(&"D");
    assert_eq!(my_value, None);
    // Cache == [C, A, X]
}


//
// ─────────────────────────────────────────────────────────────
//   TESTS POUR cache.rs
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que `new` crée un cache vide avec la bonne capacité.
#[test]
fn test_new_creates_empty_cache() {
    let cache = Cache::<i32, i32>::new(3);
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.capacity(), 3);
}

/// Vérifie que `new(0)` panique comme prévu.
#[test]
fn test_new_panics_on_zero_capacity() {
    let result = std::panic::catch_unwind(|| Cache::<i32, i32>::new(0));
    assert!(result.is_err());
}

/// Vérifie que `get` retourne une valeur existante.
#[test]
fn test_get_returns_value() {
    let mut cache = Cache::new(2);
    cache.put("A", 10);
    assert_eq!(cache.get(&"A"), Some(&10));
}

/// Vérifie que `get` retourne None si la clé n’existe pas.
#[test]
fn test_get_returns_none_for_missing_key() {
    let mut cache = Cache::<&str, i32>::new(2);
    assert_eq!(cache.get(&"X"), None);
}

/// Vérifie que `put` insère une nouvelle entrée.
#[test]
fn test_put_inserts_new_value() {
    let mut cache = Cache::new(2);
    assert_eq!(cache.put("A", 1), None);
    assert_eq!(cache.get(&"A"), Some(&1));
}

/// Vérifie que `put` met à jour une entrée existante.
#[test]
fn test_put_updates_existing_value() {
    let mut cache = Cache::new(2);
    cache.put("A", 1);
    assert_eq!(cache.put("A", 2), Some(1));
    assert_eq!(cache.get(&"A"), Some(&2));
}

/// Vérifie que `put` déclenche une éviction quand le cache est plein.
#[test]
fn test_put_eviction_occurs() {
    let mut cache = Cache::new(2);
    cache.put("A", 1);
    cache.put("B", 2);
    cache.put("C", 3); // évince A
    assert_eq!(cache.get(&"A"), None);
}

/// Vérifie que `get` déplace la clé en MRU.
#[test]
fn test_get_moves_key_to_mru() {
    let mut cache = Cache::new(2);
    cache.put("A", 1);
    cache.put("B", 2);
    cache.get(&"A"); // A devient MRU
    cache.put("C", 3); // évince B
    assert_eq!(cache.get(&"B"), None);
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS POUR persistent.rs
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que `new_persistent` recharge correctement un fichier existant.
#[test]
fn test_new_persistent_loads_existing_file() {
    let path = "test_persistent_load.txt";
    fs::write(path, "A=Marie\nB=Léane\n").unwrap();

    let mut cache = Cache::<String, String>::new_persistent(3, path);

    assert_eq!(cache.get(&"A".into()), Some(&"Marie".into()));
    assert_eq!(cache.get(&"B".into()), Some(&"Léane".into()));

    let _ = fs::remove_file(path);
}

/// Vérifie que `new_persistent` crée un cache vide si le fichier n’existe pas.
#[test]
fn test_new_persistent_empty_if_file_missing() {
    let path = "test_persistent_missing.txt";
    let _ = fs::remove_file(path);

    let cache = Cache::<String, String>::new_persistent(3, path);
    assert_eq!(cache.len(), 0);
}

/// Vérifie que `save` crée bien un fichier.
#[test]
fn test_save_creates_file() {
    let path = "test_save_file.txt";
    let _ = fs::remove_file(path);

    let mut cache = Cache::<String, String>::new(2);
    cache.put("A".into(), "Marie".into());
    cache.save(path).unwrap();

    assert!(fs::metadata(path).is_ok());
    let _ = fs::remove_file(path);
}

/// Vérifie que `save` écrit correctement les paires clé=valeur.
#[test]
fn test_save_writes_correct_content() {
    let path = "test_save_content.txt";
    let _ = fs::remove_file(path);

    let mut cache = Cache::<String, String>::new(2);
    cache.put("A".into(), "Marie".into());
    cache.put("B".into(), "Léane".into());
    cache.save(path).unwrap();

    let content = fs::read_to_string(path).unwrap();
    assert!(content.contains("A=Marie"));
    assert!(content.contains("B=Léane"));

    let _ = fs::remove_file(path);
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS POUR errors.rs
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que la conversion `From<std::io::Error>` fonctionne.
#[test]
fn test_cache_error_io_conversion() {
    let err = CacheError::from(std::io::Error::new(std::io::ErrorKind::Other, "x"));
    match err {
        CacheError::Io(_) => assert!(true),
        _ => panic!("Erreur Io attendue"),
    }
}

/// Vérifie que `CacheResult<T>` fonctionne comme un alias de Result.
#[test]
fn test_cache_result_type_alias() {
    fn returns_ok() -> CacheResult<i32> {
        Ok(42)
    }
    assert_eq!(returns_ok().unwrap(), 42);
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS POUR traits.rs
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que `get` du trait appelle bien l’implémentation interne.
#[test]
fn test_trait_get_works() {
    let mut cache = Cache::new(2);
    cache.put("A", 1);
    assert_eq!(LruCache::get(&mut cache, &"A"), Some(&1));
}

/// Vérifie que `put` du trait appelle bien l’implémentation interne.
#[test]
fn test_trait_put_works() {
    let mut cache = Cache::new(2);
    assert_eq!(LruCache::put(&mut cache, "A", 1), None);
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS POUR structs.rs
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que la structure interne démarre vide.
#[test]
fn test_structs_initial_state() {
    let cache = Cache::<i32, i32>::new(3);
    assert_eq!(cache.len(), 0);
    assert_eq!(cache.capacity(), 3);
}

/// Vérifie que les vecteurs internes ont la bonne taille.
#[test]
fn test_structs_internal_vectors_have_correct_size() {
    let cache = Cache::<i32, i32>::new(3);
    assert_eq!(cache.values.len(), 3);
    assert_eq!(cache.keys.len(), 3);
}