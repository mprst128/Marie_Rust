//! Module de gestion des erreurs du cache LRU

use std::fmt;

/// Énumération des erreurs possibles du cache
#[derive(Debug, PartialEq)]
pub enum CacheError {
    /// Erreur lors de l'écriture du fichier de persistance
    WriteError(String),
    /// Erreur lors de la lecture du fichier de persistance
    ReadError(String),
    /// Erreur de sérialisation/désérialisation
    SerializationError(String),
    /// Capacité invalide (zéro ou négative)
    InvalidCapacity,
    /// Fichier de cache corrompu
    CorruptedCache(String),
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::WriteError(msg) => write!(f, "Erreur d'écriture: {}", msg),
            CacheError::ReadError(msg) => write!(f, "Erreur de lecture: {}", msg),
            CacheError::SerializationError(msg) => {
                write!(f, "Erreur de sérialisation: {}", msg)
            }
            CacheError::InvalidCapacity => {
                write!(f, "Capacité invalide: doit être supérieure à zéro")
            }
            CacheError::CorruptedCache(msg) => write!(f, "Cache corrompu: {}", msg),
        }
    }
}

impl std::error::Error for CacheError {}

/// Type alias pour les résultats du cache
pub type CacheResult<T> = Result<T, CacheError>;