//! # Exemple d'utilisation du cache LRU persistant
//!
//! Ce programme démontre :
//! - la création d’un cache LRU générique,
//! - l’insertion de valeurs,
//! - l’éviction automatique selon la politique LRU,
//! - la persistance dans un fichier texte,
//! - le rechargement automatique du cache au démarrage.
//!
//! Le fichier utilisé pour la persistance est : `cache_data.txt`.
//!
//! # Exemple minimal
//! ```rust
//! use lru_cache::{Cache, LruCache};
//!
//! let mut cache = Cache::<String, String>::new(2);
//! cache.put("A".into(), "Marie".into());
//! cache.put("B".into(), "Léane".into());
//! assert_eq!(cache.get(&"A".into()), Some(&"Marie".into()));
//! ```

use lru_cache::Cache;
use lru_cache::LruCache;

/// Point d'entrée principal du programme.
///
/// Cette fonction :
/// — charge un cache LRU depuis un fichier (ou le crée vide),
/// — affiche son état initial,
/// — insère plusieurs valeurs pour illustrer le fonctionnement LRU,
/// — montre les évictions automatiques,
/// — sauvegarde l’état final du cache dans un fichier.
///
/// Le but est de fournir une démonstration claire et lisible du
/// fonctionnement du cache LRU **avec persistance**.
fn main() {
    println!("===============================");
    println!("     Démonstration LRU Cache   ");
    println!("        avec persistance       ");
    println!("===============================\n");

    // Fichier utilisé pour stocker le cache entre deux exécutions.
    let path = "cache_data.txt";

    // -------------------------------------------------------------------------
    // Chargement du cache depuis un fichier (ou création si le fichier n'existe pas)
    // -------------------------------------------------------------------------
    println!("Chargement du cache depuis {path:?} ...");
    let mut cache = Cache::<String, String>::new_persistent(3, path);

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
    println!("\n--- Ajout de D (éviction de A) ---");
    cache.put("D".to_string(), "Nicolas".to_string());

    println!("A = {:?}", cache.get(&"A".to_string())); // None
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
    match cache.save(path) {
        Ok(_) => println!("Cache sauvegardé dans {path:?}"),
        Err(e) => println!("Erreur lors de la sauvegarde : {e:?}"),
    }

    println!("\n===============================");
    println!("        Fin de la démo         ");
    println!("===============================");
}