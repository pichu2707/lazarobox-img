//! Navegador de archivos compartido por las pantallas que necesitan una ruta.
//!
//! Mantiene la carpeta actual, su listado (subcarpetas + imágenes) y el
//! elemento resaltado. No renderiza: solo modela el estado y la navegación.
//! El controlador lo manipula y las pantallas lo leen.

use std::fs;
use std::path::{Path, PathBuf};

/// Extensiones consideradas imágenes en el listado.
const IMAGE_EXTS: [&str; 6] = ["jpg", "jpeg", "png", "webp", "bmp", "tiff"];

/// Una entrada del listado: subcarpeta, imagen o el ascenso "..".
#[derive(Debug, Clone)]
pub struct BrowserEntry {
    /// Nombre mostrado.
    pub name: String,
    /// Ruta absoluta del elemento.
    pub path: PathBuf,
    /// Verdadero si es una carpeta (incluye "..").
    pub is_dir: bool,
    /// Verdadero si es la entrada de ascenso "..".
    pub is_parent: bool,
}

/// Estado del navegador de archivos.
#[derive(Debug)]
pub struct Browser {
    /// Carpeta que se está mostrando.
    pub current_dir: PathBuf,
    /// Entradas visibles: "..", subcarpetas y luego imágenes.
    pub entries: Vec<BrowserEntry>,
    /// Índice del elemento resaltado.
    pub selected: usize,
}

impl Default for Browser {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut browser = Self {
            current_dir,
            entries: Vec::new(),
            selected: 0,
        };
        browser.refresh();
        browser
    }
}

impl Browser {
    /// Vuelve a leer la carpeta actual y reconstruye el listado.
    pub fn refresh(&mut self) {
        let mut entries = Vec::new();

        if let Some(parent) = self.current_dir.parent() {
            entries.push(BrowserEntry {
                name: "..".to_string(),
                path: parent.to_path_buf(),
                is_dir: true,
                is_parent: true,
            });
        }

        if let Ok(read) = fs::read_dir(&self.current_dir) {
            let mut dirs = Vec::new();
            let mut files = Vec::new();

            for entry in read.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                // Se omiten los ocultos.
                if name.starts_with('.') {
                    continue;
                }

                if path.is_dir() {
                    dirs.push(BrowserEntry {
                        name,
                        path,
                        is_dir: true,
                        is_parent: false,
                    });
                } else if is_image(&path) {
                    files.push(BrowserEntry {
                        name,
                        path,
                        is_dir: false,
                        is_parent: false,
                    });
                }
            }

            dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            entries.extend(dirs);
            entries.extend(files);
        }

        self.entries = entries;

        if self.selected >= self.entries.len() {
            self.selected = self.entries.len().saturating_sub(1);
        }
    }

    /// Mueve la selección una posición hacia abajo, con wrap.
    pub fn move_down(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.entries.len();
    }

    /// Mueve la selección una posición hacia arriba, con wrap.
    pub fn move_up(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        let count = self.entries.len();
        self.selected = (self.selected + count - 1) % count;
    }

    /// Entrada actualmente resaltada.
    pub fn selected_entry(&self) -> Option<&BrowserEntry> {
        self.entries.get(self.selected)
    }

    /// Si la entrada resaltada es una carpeta, entra en ella.
    pub fn enter_selected(&mut self) {
        if let Some(entry) = self.entries.get(self.selected) {
            if entry.is_dir {
                self.current_dir = entry.path.clone();
                self.selected = 0;
                self.refresh();
            }
        }
    }

    /// Sube a la carpeta padre, si existe.
    pub fn to_parent(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.selected = 0;
            self.refresh();
        }
    }
}

/// Comprueba si la ruta tiene una extensión de imagen conocida.
fn is_image(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => IMAGE_EXTS.contains(&ext.to_lowercase().as_str()),
        None => false,
    }
}
