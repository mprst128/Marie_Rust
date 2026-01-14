/// # Exemple d'utilisation du cache LRU persistant
///
/// Ce programme démontre :
/// - la création d’un cache LRU générique
/// - l’insertion de valeurs
/// - l’éviction automatique selon la politique LRU
/// - la persistance dans un fichier texte
/// - le rechargement automatique du cache au démarrage
///
/// Le fichier utilisé pour la persistance est : `cache_data.txt`
///


use lru_cache::Cache;
use lru_cache::persistent::persistent;

/// # Exemple d'utilisation
/// ```rust
/// use lru_cache::{Cache, LruCache};
///
/// let mut cache = Cache::<String, String>::new(2);
/// cache.put("A".into(), "Marie".into());
/// cache.put("B".into(), "Léane".into());
/// assert_eq!(cache.get(&"A".into()), Some(&"Marie".into()));
/// ```


fn main() {
    println!("===============================");
    println!("     Démonstration LRU Cache   ");
    println!("        avec persistance       ");
    println!("===============================\n");

    // Fichier utilisé pour stocker le cache entre deux exécutions
    let path = "cache_data.txt";

    // -------------------------------------------------------------------------
    // Chargement du cache depuis un fichier
    // -------------------------------------------------------------------------
    println!("Chargement du cache depuis {path:?} ...");

    let mut cache = Cache::<String, String>::new(3);

    // On tente de charger le fichier
    match persistent::load(path, &mut cache) {
        Ok(_) => println!("Cache chargé depuis {path:?}"),
        Err(_) => println!("Aucun fichier trouvé, cache initialisé vide"),
    }

    println!("État initial du cache :");
    println!("A = {:?}", cache.get(&"A".to_string()));
    println!("B = {:?}", cache.get(&"B".to_string()));
    println!("C = {:?}", cache.get(&"C".to_string()));
    println!("Taille : {}", cache.len());
    println!("Capacité : {}", cache.capacity());

    // -------------------------------------------------------------------------
    // Insertion de valeurs
    // -------------------------------------------------------------------------
    println!("\n--- Insertion des valeurs ---");
    cache.put("A".to_string(), "Marie".to_string());
    cache.put("B".to_string(), "Léane".to_string());
    cache.put("C".to_string(), "Maia".to_string());

    println!("Cache après A, B, C :");
    println!("A = {:?}", cache.get(&"A".to_string()));
    println!("B = {:?}", cache.get(&"B".to_string()));
    println!("C = {:?}", cache.get(&"C".to_string()));
    println!("Taille actuelle : {}", cache.len());

    // -------------------------------------------------------------------------
    // Ajout d'une nouvelle valeur → éviction du LRU
    // -------------------------------------------------------------------------
    println!("\n--- Ajout de D (éviction du LRU) ---");
    cache.put("D".to_string(), "Nicolas".to_string());

    println!("A = {:?}", cache.get(&"A".to_string()));
    println!("B = {:?}", cache.get(&"B".to_string()));
    println!("C = {:?}", cache.get(&"C".to_string()));
    println!("D = {:?}", cache.get(&"D".to_string()));

    // -------------------------------------------------------------------------
    // Accès à une valeur → devient MRU
    // -------------------------------------------------------------------------
    println!("\n--- Accès à B (devient MRU) ---");
    println!("B = {:?}", cache.get(&"B".to_string()));

    // -------------------------------------------------------------------------
    // Nouvelle insertion → éviction du LRU actuel
    // -------------------------------------------------------------------------
    println!("\n--- Ajout de X (éviction du LRU) ---");
    cache.put("X".to_string(), "Noa".to_string());

    println!("B = {:?}", cache.get(&"B".to_string()));
    println!("C = {:?}", cache.get(&"C".to_string()));
    println!("D = {:?}", cache.get(&"D".to_string()));
    println!("X = {:?}", cache.get(&"X".to_string()));

    // -------------------------------------------------------------------------
    // État final
    // -------------------------------------------------------------------------
    println!("\n--- État final ---");
    println!("Taille : {}", cache.len());
    println!("Capacité : {}", cache.capacity());

    // -------------------------------------------------------------------------
    // Sauvegarde du cache dans le fichier
    // -------------------------------------------------------------------------
    println!("\n--- Sauvegarde du cache ---");
    match persistent::save(path, &cache) {
        Ok(_) => println!("Cache sauvegardé dans {path:?}"),
        Err(e) => println!("Erreur lors de la sauvegarde : {e:?}"),
    }

    println!("\n===============================");
    println!("        Fin de la démo         ");
    println!("===============================");
}