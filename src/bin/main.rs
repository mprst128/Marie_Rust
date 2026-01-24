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
    // Chargement du cache depuis un fichier pour récupération
    // -------------------------------------------------------------------------
    println!("Chargement du cache depuis {path:?} ...");

    let mut loaded_cache = Cache::<String, String>::new(3);

    // On tente de charger le fichier
    match persistent::load(path, &mut loaded_cache) {
        Ok(_) => println!("Cache chargé depuis {path:?}"),
        Err(_) => println!("Aucun fichier trouvé"),
    }

    // Créer un nouveau cache vide pour l'affichage (simule un cache vidé)
    let mut empty_cache = Cache::<String, String>::new(3);

    println!("État du cache chargé :");
    println!("A = {:?}", empty_cache.get(&"A".to_string()));
    println!("B = {:?}", empty_cache.get(&"B".to_string()));
    println!("C = {:?}", empty_cache.get(&"C".to_string()));
    println!("Taille : {}", empty_cache.len());
    println!("Capacité : {}", empty_cache.capacity());

    // -------------------------------------------------------------------------
    // Insertion de valeurs
    // -------------------------------------------------------------------------
    println!("\n--- Insertion des valeurs ---");
    empty_cache.put("A".to_string(), "Marie".to_string());
    empty_cache.put("B".to_string(), "Léane".to_string());
    empty_cache.put("C".to_string(), "Maia".to_string());

    println!("Cache après A, B, C :");
    println!("A = {:?}", empty_cache.get(&"A".to_string()));
    println!("B = {:?}", empty_cache.get(&"B".to_string()));
    println!("C = {:?}", empty_cache.get(&"C".to_string()));
    println!("Taille actuelle : {}", empty_cache.len());

    // -------------------------------------------------------------------------
    // Ajout d'une nouvelle valeur → éviction du LRU
    // -------------------------------------------------------------------------
    println!("\n--- Ajout de D (éviction du LRU) ---");
    empty_cache.put("D".to_string(), "Nicolas".to_string());

    println!("A = {:?}", empty_cache.get(&"A".to_string()));
    println!("B = {:?}", empty_cache.get(&"B".to_string()));
    println!("C = {:?}", empty_cache.get(&"C".to_string()));
    println!("D = {:?}", empty_cache.get(&"D".to_string()));

    // -------------------------------------------------------------------------
    // Accès à une valeur → devient MRU
    // -------------------------------------------------------------------------
    println!("\n--- Accès à B (devient MRU) ---");
    println!("B = {:?}", empty_cache.get(&"B".to_string()));

    // -------------------------------------------------------------------------
    // Nouvelle insertion → éviction du LRU actuel
    // -------------------------------------------------------------------------
    println!("\n--- Ajout de X (éviction du LRU) ---");
    empty_cache.put("X".to_string(), "Noa".to_string());

    println!("B = {:?}", empty_cache.get(&"B".to_string()));
    println!("C = {:?}", empty_cache.get(&"C".to_string()));
    println!("D = {:?}", empty_cache.get(&"D".to_string()));
    println!("X = {:?}", empty_cache.get(&"X".to_string()));

    // -------------------------------------------------------------------------
    // État final
    // -------------------------------------------------------------------------
    println!("\n--- État final ---");
    println!("Taille : {}", empty_cache.len());
    println!("Capacité : {}", empty_cache.capacity());

    // -------------------------------------------------------------------------
    // Sauvegarde du cache dans le fichier
    // -------------------------------------------------------------------------
    println!("\n--- Sauvegarde du cache ---");
    match persistent::save(path, &empty_cache) {
        Ok(_) => println!("Cache sauvegardé dans {path:?}"),
        Err(e) => println!("Erreur lors de la sauvegarde : {e:?}"),
    }

    println!("\n===============================");
    println!("        Fin de la démo         ");
    println!("===============================");
}