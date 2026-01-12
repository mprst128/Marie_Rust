use lru_cache::Cache;

fn main() {
    // Création d’un cache LRU de capacité 3
    let mut cache = Cache::new(3);

    println!("--- Insertion des valeurs ---");
    cache.put("A", "Marie");
    cache.put("B", "Léane");
    cache.put("C", "Maia");

    println!("Cache après A, B, C :");
    println!("A = {:?}", cache.get(&"A"));
    println!("B = {:?}", cache.get(&"B"));
    println!("C = {:?}", cache.get(&"C"));
    println!("Taille actuelle : {}", cache.len());

    println!("\n--- Ajout de D (éviction de A) ---");
    cache.put("D", "Nicolas");

    println!("A = {:?}", cache.get(&"A")); // None
    println!("B = {:?}", cache.get(&"B"));
    println!("C = {:?}", cache.get(&"C"));
    println!("D = {:?}", cache.get(&"D"));

    println!("\n--- Accès à B (devient MRU) ---");
    println!("B = {:?}", cache.get(&"B"));

    println!("\n--- Ajout de X (éviction du LRU) ---");
    cache.put("X", "Noa");

    println!("B = {:?}", cache.get(&"B"));
    println!("C = {:?}", cache.get(&"C"));
    println!("D = {:?}", cache.get(&"D"));
    println!("X = {:?}", cache.get(&"X"));

    println!("\n--- État final ---");
    println!("Taille : {}", cache.len());
    println!("Capacité : {}", cache.size());
}