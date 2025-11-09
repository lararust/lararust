use std::{fs, path::Path};

/// Saves a compiled template to the cache folder.
pub fn cache_view(name: &str, content: &str) {
    let dir = Path::new("storage/framework/views");
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Error creating cache directory: {}", e);
        return;
    }

    let path = dir.join(format!("{}.html", name));
    if let Err(e) = fs::write(&path, content) {
        eprintln!("Error caching view '{}': {}", name, e);
    } else {
        println!("🗂️ Cached view: {}", path.display());
    }
}

/// Loads a cached view if it exists.
pub fn get_cached(name: &str) -> Option<String> {
    let path = format!("storage/framework/views/{}.html", name);
    if Path::new(&path).exists() {
        match fs::read_to_string(&path) {
            Ok(content) => Some(content),
            Err(e) => {
                eprintln!("Error reading cached view '{}': {}", name, e);
                None
            }
        }
    } else {
        None
    }
}
