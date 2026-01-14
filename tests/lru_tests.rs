use lru_cache::{Cache, LruCache};
use std::fs;

//
// ─────────────────────────────────────────────────────────────
//   TESTS D’INTÉGRATION
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que le cache insère correctement plusieurs valeurs
/// et que les accès fonctionnent comme prévu
#[test]
fn test_lru_basic_insertion_and_access() {
    let mut cache = Cache::new(3);

    cache.put("A", "Marie");
    cache.put("B", "Léane");
    cache.put("C", "Maia");

    assert_eq!(cache.get(&"A"), Some(&"Marie"));
    assert_eq!(cache.get(&"B"), Some(&"Léane"));
    assert_eq!(cache.get(&"C"), Some(&"Maia"));
}

/// Vérifie que l’éviction LRU fonctionne sur un scénario
#[test]
fn test_lru_eviction_simple() {
    let mut cache = Cache::new(2);

    cache.put("A", 1);
    cache.put("B", 2);
    cache.put("C", 3); // évince A

    assert_eq!(cache.get(&"A"), None);
    assert_eq!(cache.get(&"B"), Some(&2));
    assert_eq!(cache.get(&"C"), Some(&3));
}

/// Vérifie que l’accès à une clé la rend MRU
/// et influence l’ordre d’éviction
#[test]
fn test_lru_access_updates_order() {
    let mut cache = Cache::new(2);

    cache.put("A", 1);
    cache.put("B", 2);

    cache.get(&"A"); // A devient MRU

    cache.put("C", 3); // évince B

    assert_eq!(cache.get(&"B"), None);
    assert_eq!(cache.get(&"A"), Some(&1));
    assert_eq!(cache.get(&"C"), Some(&3));
}

/// Vérifie que la mise à jour d’une clé existante
/// ne change pas la taille et conserve l’ordre LRU
#[test]
fn test_lru_update_existing_key() {
    let mut cache = Cache::new(2);

    cache.put("A", 1);
    cache.put("B", 2);

    let old = cache.put("A", 99);
    assert_eq!(old, Some(1));

    // A devient MRU
    cache.put("C", 3); // évince B

    assert_eq!(cache.get(&"B"), None);
    assert_eq!(cache.get(&"A"), Some(&99));
    assert_eq!(cache.get(&"C"), Some(&3));
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS D’INTÉGRATION : TRAIT LruCache
// ─────────────────────────────────────────────────────────────
//


/// Vérifie que len() et capacity() du trait fonctionnent
#[test]
fn test_trait_len_capacity() {
    let mut cache = Cache::new(2);

    cache.put("A", 1);
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.capacity(), 2);
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS D’INTÉGRATION : PERSISTENCE
// ─────────────────────────────────────────────────────────────
//

/// Vérifie un cycle complet :
/// - création d’un cache
/// - insertion
/// - sauvegarde
/// - rechargement
/// - vérification des valeurs
#[test]
fn test_persistence_full_cycle() {
    let path = "test_full_cycle.txt";
    let _ = fs::remove_file(path);

    {
        let mut cache = Cache::<String, String>::new_persistent(3, path);
        cache.put("A".into(), "Marie".into());
        cache.put("B".into(), "Léane".into());
        cache.save(path).unwrap();
    }

    {
        let mut cache = Cache::<String, String>::new_persistent(3, path);
        assert_eq!(cache.get(&"A".into()), Some(&"Marie".into()));
        assert_eq!(cache.get(&"B".into()), Some(&"Léane".into()));
    }

    let _ = fs::remove_file(path);
}

/// Vérifie que l’éviction est bien persistée dans le fichier
#[test]
fn test_persistence_eviction_is_saved() {
    let path = "test_eviction_persisted.txt";
    let _ = fs::remove_file(path);

    {
        let mut cache = Cache::<String, String>::new_persistent(2, path);
        cache.put("A".into(), "Marie".into());
        cache.put("B".into(), "Léane".into());
        cache.put("C".into(), "Maia".into()); // évince A
        cache.save(path).unwrap();
    }

    {
        let mut cache = Cache::<String, String>::new_persistent(2, path);
        assert_eq!(cache.get(&"A".into()), None);
        assert_eq!(cache.get(&"B".into()), Some(&"Léane".into()));
        assert_eq!(cache.get(&"C".into()), Some(&"Maia".into()));
    }

    let _ = fs::remove_file(path);
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS D’INTÉGRATION : ROBUSTESSE
// ─────────────────────────────────────────────────────────────
//

/// Vérifie que le cache supporte des clés/valeurs génériques
#[test]
fn test_generic_types_integration() {
    #[derive(Clone, Hash, PartialEq, Eq, Debug)]
    struct Key(u32);

    #[derive(Clone, Debug, PartialEq)]
    struct Value(&'static str);

    let mut cache = Cache::new(2);
    cache.put(Key(1), Value("Marie"));
    cache.put(Key(2), Value("Léane"));

    assert_eq!(cache.get(&Key(1)), Some(&Value("Marie")));
    assert_eq!(cache.get(&Key(2)), Some(&Value("Léane")));
}

/// Vérifie que le cache reste cohérent après plusieurs opérations mixtes
#[test]
fn test_complex_sequence() {
    let mut cache = Cache::new(3);

    cache.put("A", 1);
    cache.put("B", 2);
    cache.put("C", 3);

    cache.get(&"A"); // A devient MRU
    cache.put("D", 4); // évince B

    assert_eq!(cache.get(&"B"), None);
    assert_eq!(cache.get(&"A"), Some(&1));
    assert_eq!(cache.get(&"C"), Some(&3));
    assert_eq!(cache.get(&"D"), Some(&4));
}