use lru_cache::Cache;

//
// ─────────────────────────────────────────────────────────────
//   TEST PROF
// ─────────────────────────────────────────────────────────────
//

#[test]
fn test_lru_cache() {
    let mut cache = Cache::new(3); // Taille de 3

    // put() avec nouvelle clé retourne None
    let previous_value = cache.put("A".to_string(), String::from("Marie"));
    assert!(previous_value.is_none());

    cache.put("B".to_string(), String::from("Léane"));
    cache.put("C".to_string(), String::from("Maia"));
    cache.put("D".to_string(), String::from("Nicolas"));
    // Cache == [B, C, D]

    assert_eq!(cache.get(&"A".to_string()), None);

    assert_eq!(cache.get(&"D".to_string()), Some(&String::from("Nicolas")));
    // Cache == [B, C, D]

    assert_eq!(cache.get(&"B".to_string()), Some(&String::from("Léane")));
    // Cache == [C, D, B]

    assert_eq!(cache.get(&"C".to_string()), Some(&String::from("Maia")));
    // Cache == [D, B, C]

    assert_eq!(cache.get(&"X".to_string()), None);
    // Cache == [D, B, C]

    cache.put("A".to_string(), String::from("Marie"));
    // Cache == [B, C, A]

    let my_previous_value_a = cache.put("A".to_string(), String::from("Marie"));
    assert_eq!(my_previous_value_a, Some(String::from("Marie")));
    cache.put("X".to_string(), String::from("Noa"));
    // Cache == [C, A, X]

    assert_eq!(cache.get(&"B".to_string()), None);
    assert_eq!(cache.get(&"D".to_string()), None);
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS INITIAUX
// ─────────────────────────────────────────────────────────────
//

#[test]
fn test_insert_and_get() {
    let mut cache = Cache::new(2);
    cache.put("A", 10);
    cache.put("B", 20);

    assert_eq!(cache.get(&"A"), Some(&10));
    assert_eq!(cache.get(&"B"), Some(&20));
}

#[test]
fn test_update_existing_key() {
    let mut cache = Cache::new(2);
    cache.put("A", 10);
    let old = cache.put("A", 99);

    assert_eq!(old, Some(10));
    assert_eq!(cache.get(&"A"), Some(&99));
    assert_eq!(cache.len(), 1);
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS LRU
// ─────────────────────────────────────────────────────────────
//

#[test]
fn test_lru_eviction() {
    let mut cache = Cache::new(2);
    cache.put("A", 1);
    cache.put("B", 2);
    cache.put("C", 3); // évince A

    assert_eq!(cache.get(&"A"), None);
    assert_eq!(cache.get(&"B"), Some(&2));
    assert_eq!(cache.get(&"C"), Some(&3));
}

#[test]
fn test_lru_order_after_get() {
    let mut cache = Cache::new(3);
    cache.put("A", 1);
    cache.put("B", 2);
    cache.put("C", 3);

    assert_eq!(cache.get(&"A"), Some(&1)); // A devient MRU

    cache.put("D", 4); // évince B

    assert_eq!(cache.get(&"B"), None);
    assert_eq!(cache.get(&"C"), Some(&3));
    assert_eq!(cache.get(&"A"), Some(&1));
    assert_eq!(cache.get(&"D"), Some(&4));
}

//
// ─────────────────────────────────────────────────────────────
//   TESTS GÉNÉRIQUES
// ─────────────────────────────────────────────────────────────
//

#[test]
fn test_generic_types() {
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

//
// ─────────────────────────────────────────────────────────────
//   TESTS CAPACITÉ
// ─────────────────────────────────────────────────────────────
//

#[test]
fn test_capacity() {
    let cache = Cache::<&str, i32>::new(5);
    assert_eq!(cache.size(), 5);
    assert!(cache.is_empty());
}

//
// ─────────────────────────────────────────────────────────────
//   AUTRES TESTS
// ─────────────────────────────────────────────────────────────
//

#[test]
fn test_internal_links_after_multiple_operations() {
    let mut cache = Cache::new(3);

    cache.put("A", 1);
    cache.put("B", 2);
    cache.put("C", 3);

    cache.get(&"A"); // A devient MRU
    cache.put("D", 4); // évince B

    assert_eq!(cache.get(&"B"), None);
    assert_eq!(cache.get(&"C"), Some(&3));
    assert_eq!(cache.get(&"A"), Some(&1));
    assert_eq!(cache.get(&"D"), Some(&4));
}

#[test]
fn test_eviction_does_not_break_list() {
    let mut cache = Cache::new(2);

    cache.put("A", 1);
    cache.put("B", 2);
    cache.put("C", 3); // évince A

    assert_eq!(cache.get(&"B"), Some(&2)); // B devient MRU
    assert_eq!(cache.get(&"C"), Some(&3)); // C devient MRU

    cache.put("D", 4); // évince B (LRU)

    assert_eq!(cache.get(&"B"), None);
    assert_eq!(cache.get(&"C"), Some(&3)); // C est toujours là
    assert_eq!(cache.get(&"D"), Some(&4));
}