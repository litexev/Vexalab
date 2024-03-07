use lazy_static::lazy_static;
use macroquad::texture::{FilterMode, Texture2D};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;

lazy_static! {
    static ref IMAGE_CACHE: Mutex<HashMap<String, Texture2D>> = Mutex::new(HashMap::new());
}

fn load_bytes_from_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

// Function to get a texture from the cache or load it if not present
pub fn get_texture(path: &str) -> Texture2D {
    let mut cache = IMAGE_CACHE.lock().unwrap();
    if let Some(texture) = cache.get(path) {
        return texture.clone();
    }
    let bytes = load_bytes_from_file(path).unwrap();
    let texture = Texture2D::from_file_with_format(bytes.as_slice(), None);
    texture.set_filter(FilterMode::Nearest);
    cache.insert(path.to_string(), texture.clone());
    return texture;
}
