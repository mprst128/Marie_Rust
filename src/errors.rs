/// Erreurs possibles lors de l'utilisation du cache
///
/// Cette enum couvre :
/// - les erreurs d'entrée/sortie (fichiers)
/// - les erreurs de parsing lors du chargement persistant
#[derive(Debug)]
pub enum CacheError {
    /// Erreur liée au système de fichiers (lecture/écriture)
    Io(std::io::Error),

    /// Impossible de parser une clé depuis une ligne du fichier
    ParseKey,

    /// Impossible de parser une valeur depuis une ligne du fichier
    ParseValue,
}

/// Alias pratique pour retourner un résultat lié au cache
pub type CacheResult<T> = Result<T, CacheError>;

/// Conversion automatique d'une erreur d'E/S vers `CacheError`
impl From<std::io::Error> for CacheError {
    fn from(err: std::io::Error) -> Self {
        CacheError::Io(err)
    }
}