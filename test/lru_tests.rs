use lru_cache::Cache;

#[test]
fn test_lru_cache_integration() {
    let mut cache = Cache::new(3);

    cache.put("A", "Marie");
    cache.put("B", "Léane");
    cache.put("C", "Maia");
    cache.put("D", "Nicolas");

    assert_eq!(cache.get(&"A"), None);
    assert_eq!(cache.get(&"D"), Some(&"Nicolas"));
}

