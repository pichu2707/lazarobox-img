//! Estado global de la aplicación LazaroBox Image.
//! app/state.rs
//!
//! Este módulo conteine todo el estado global de la apliación.
//!
//! Ninguna pantalla debe almacenar información propia;
//! toda la información persistente vive en AppState.
//!
//! El controlador es el únicao encargvado de modificar este estado.
//!
//! AppState es el punjto único de verdad (Single Source of Truth) de la aplicación.
//! Todas las pantallas leen o modifican este estado a través del controlador.
//!
//! La TUI nunca accederá directamente al motor de optimización i a los módulos
//! de matadata; siempre trabajará sobre este estado.
//!
use crate::app::browser::Browser;
use crate::converter::Conversion;
use crate::inspector::ImageInfo;
use crate::metadata::ImageMetadata;
use crate::types::OutputFormat;
use crate::update::{CURRENT_VERSION, UpdateStatus};
use std::path::PathBuf;

/// Estado global de la aplicación.
#[derive(Debug, Default)]
pub struct AppState {
    /// Pantalla actualmente visible.
    pub screen: Screen,
    /// Índice de la opción resaltada en el menú actual.
    pub selected: usize,
    /// Inforamaicón del proyecto abierto.
    pub project: ProjectState,
    /// Configuración de los metadatos.
    pub metadata: MetadataState,
    /// Estado de la pantalla de optimización.
    pub optimize: OptimizeState,
    /// Estado de la pantalla de inspección.
    pub inspect: InspectState,
    /// Estado de la pantalla de conversión.
    pub convert: ConvertState,
    /// Estado de la pantalla de metadatos.
    pub metadata_view: MetadataView,
    /// Navegador de archivos compartido por las pantallas.
    pub browser: Browser,
    /// Estado de comprobación de actualizaciones.
    pub update: UpdateState,
}

/// Pantallas disponibles dentro de la aplicación.
///
/// Modelo feature-based: cada variante es una vista de nivel superior.
/// Los pasos internos de un flujo (p. ej. el wizard de optimización)
/// se modelarán como sub-estado de su pantalla, no como variantes aquí.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    /// Pantalla principal con el menú.
    #[default]
    Home,
    /// Optimización de imágenes.
    Optimize,
    /// Gestión de metadatos.
    Metadata,
    /// Conversión de formatos.
    Convert,
    /// Inspección de una imagen.
    Inspect,
    /// Ajustes de la aplicación.
    Settings,
}

/// Ítems del menú principal (pantalla Home).
///
/// Fuente única de verdad del menú: el widget lo renderiza y el controlador
/// navega sobre él. El orden de `ALL` define el orden en pantalla.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    Optimize,
    Metadata,
    Convert,
    Inspect,
    Settings,
    Exit,
}

impl MenuItem {
    /// Todos los ítems, en orden de aparición.
    pub const ALL: [MenuItem; 6] = [
        MenuItem::Optimize,
        MenuItem::Metadata,
        MenuItem::Convert,
        MenuItem::Inspect,
        MenuItem::Settings,
        MenuItem::Exit,
    ];

    /// Etiqueta visible del ítem.
    pub fn label(self) -> &'static str {
        match self {
            MenuItem::Optimize => "Optimizar imágenes",
            MenuItem::Metadata => "Metadatos",
            MenuItem::Convert => "Convertir formatos",
            MenuItem::Inspect => "Inspeccionar imagen",
            MenuItem::Settings => "Ajustes",
            MenuItem::Exit => "Salir",
        }
    }

    /// Descripción breve de lo que hace el ítem (ayuda contextual).
    pub fn description(self) -> &'static str {
        match self {
            MenuItem::Optimize => "Redimensiona y comprime imágenes (WebP, PNG o JPEG).",
            MenuItem::Metadata => "Lee y edita metadatos: GPS, alt text y quitar IA.",
            MenuItem::Convert => "Convierte una imagen entre WebP, PNG y JPEG.",
            MenuItem::Inspect => "Muestra dimensiones, tamaño y formato de una imagen.",
            MenuItem::Settings => "Ajustes por defecto de la aplicación.",
            MenuItem::Exit => "Cierra la aplicación.",
        }
    }

    /// Pantalla destino al activar el ítem.
    /// `None` indica que no navega (salir de la aplicación).
    pub fn target(self) -> Option<Screen> {
        match self {
            MenuItem::Optimize => Some(Screen::Optimize),
            MenuItem::Metadata => Some(Screen::Metadata),
            MenuItem::Convert => Some(Screen::Convert),
            MenuItem::Inspect => Some(Screen::Inspect),
            MenuItem::Settings => Some(Screen::Settings),
            MenuItem::Exit => None,
        }
    }
}

/// Estado del proyecto actual.
#[derive(Debug, Default)]
pub struct ProjectState {
    /// Imagen o carpeta seleccionada.
    pub input: Option<PathBuf>,
    /// Carpeta donde se exportarán los resutados.
    pub output: Option<PathBuf>,
    /// Número de imágenes detectadas.
    pub images: Vec<PathBuf>,
}

/// Opciones de tratamiento de metadatos.
///
/// Este estado crecerá conforme se incorporen nuevos perfiles
/// (SEO, privacidad, derechos, IA, etc...)

#[derive(Debug, Default)]
pub struct MetadataState {
    /// Eliminar metadatos relacionados con IA.
    pub remove_ai: bool,
    /// Eliminar coordenadas de GPS.
    pub remove_gps: bool,
    /// Mantener el autor.
    pub preserve_author: bool,
    /// Mantener el copyright.
    pub preserve_copyright: bool,
}

/// Estado de la pantalla de inspección de una imagen.
#[derive(Debug, Default)]
pub struct InspectState {
    /// Resultado de la última inspección correcta.
    pub result: Option<ImageInfo>,
    /// Mensaje de error de la última inspección fallida.
    pub error: Option<String>,
    /// Verdadero cuando se muestra el resultado en lugar del navegador.
    pub viewing: bool,
    /// Ruta pendiente de procesar (mientras se muestra "Procesando…").
    pub pending: Option<PathBuf>,
}

/// Estado de la pantalla de conversión de formato.
#[derive(Debug, Default)]
pub struct ConvertState {
    /// Formato de destino seleccionado.
    pub format: OutputFormat,
    /// Resultado de la última conversión correcta.
    pub result: Option<Conversion>,
    /// Mensaje de error de la última conversión fallida.
    pub error: Option<String>,
    /// Verdadero cuando se muestra el resultado en lugar del navegador.
    pub viewing: bool,
    /// Ruta pendiente de convertir (mientras se muestra "Procesando…").
    pub pending: Option<PathBuf>,
}

/// Estado de la comprobación de actualizaciones.
#[derive(Debug)]
pub struct UpdateState {
    /// Versión actual del binario.
    pub current_version: &'static str,
    /// Última versión estable publicada, si ya se consultó.
    pub latest_version: Option<String>,
    /// Estado visible de la comprobación.
    pub status: UpdateStatus,
    /// Mensaje de error de la última comprobación fallida.
    pub error: Option<String>,
}

impl Default for UpdateState {
    fn default() -> Self {
        Self {
            current_version: CURRENT_VERSION,
            latest_version: None,
            status: UpdateStatus::NotChecked,
            error: None,
        }
    }
}

/// Campo enfocado en el formulario de edición de metadatos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MetaField {
    /// Alt text / descripción (se escribe).
    #[default]
    AltText,
    /// Latitud GPS en grados decimales (se escribe).
    Lat,
    /// Longitud GPS en grados decimales (se escribe).
    Lon,
    /// Quitar GPS (se alterna).
    RemoveGps,
    /// Quitar metadatos de IA (se alterna).
    RemoveAi,
    /// Acción de guardar.
    Save,
}

/// Estado de la pantalla de metadatos (lectura y edición).
#[derive(Debug, Default)]
pub struct MetadataView {
    /// Metadatos leídos de la última imagen.
    pub result: Option<ImageMetadata>,
    /// Mensaje de error de la última lectura/escritura fallida.
    pub error: Option<String>,
    /// Verdadero cuando se muestra el resultado en lugar del navegador.
    pub viewing: bool,
    /// Ruta pendiente de leer (mientras se muestra "Procesando…").
    pub pending: Option<PathBuf>,
    /// Ruta de la imagen actualmente cargada (para editar).
    pub path: Option<PathBuf>,
    /// Verdadero cuando se muestra el formulario de edición.
    pub editing: bool,
    /// Campo de alt text del formulario.
    pub alt_input: String,
    /// Campo de latitud del formulario.
    pub lat_input: String,
    /// Campo de longitud del formulario.
    pub lon_input: String,
    /// Alternador: quitar GPS.
    pub remove_gps: bool,
    /// Alternador: quitar metadatos de IA.
    pub remove_ai: bool,
    /// Campo enfocado del formulario.
    pub focus: MetaField,
    /// Mensaje de éxito tras guardar.
    pub saved: Option<String>,
}

/// Paso actual dentro del flujo de optimización.
///
/// El wizard de Optimize vive como sub-estado de su pantalla, en lugar de
/// como variantes de [`Screen`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptimizeStep {
    /// Selección de la imagen o carpeta de entrada.
    #[default]
    Input,
    /// Configuración de anchura, alto, calidad y formato.
    Config,
    /// Procesando imágenes (con barra de progreso).
    Running,
    /// Resultados de la optimización.
    Results,
}

/// Campo enfocado durante el paso de configuración.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptimizeField {
    /// Anchura máxima (se escribe con dígitos o se ajusta con ← →).
    #[default]
    Width,
    /// Alto máximo (se escribe con dígitos o se ajusta con ← →).
    Height,
    /// Calidad de salida (se ajusta con ← →).
    Quality,
    /// Formato de salida (se ajusta con ← →).
    Format,
}

/// Progreso en curso de una optimización por lotes.
#[derive(Debug, Default)]
pub struct OptimizeProgress {
    /// Imágenes que faltan por procesar.
    pub queue: Vec<PathBuf>,
    /// Imágenes ya procesadas.
    pub done: usize,
    /// Total de imágenes a procesar.
    pub total: usize,
    /// Nombre del archivo en curso.
    pub current_file: String,
    /// Suma acumulada de tamaños originales.
    pub acc_original: u64,
    /// Suma acumulada de tamaños optimizados.
    pub acc_optimized: u64,
    /// Carpeta de salida.
    pub output_dir: PathBuf,
}

/// Resumen agregado de una optimización completada.
#[derive(Debug)]
pub struct OptimizeSummary {
    /// Número de imágenes procesadas.
    pub count: usize,
    /// Suma de tamaños originales en bytes.
    pub original_total: u64,
    /// Suma de tamaños optimizados en bytes.
    pub optimized_total: u64,
    /// Carpeta donde se escribieron los resultados.
    pub output_dir: PathBuf,
}

/// Estado de la pantalla de optimización.
///
/// Reúne el paso del wizard, la entrada, la configuración editable y los
/// resultados de la última ejecución.
#[derive(Debug)]
pub struct OptimizeState {
    /// Paso actual del flujo.
    pub step: OptimizeStep,
    /// Ruta de entrada elegida en el navegador.
    pub input: String,
    /// Anchura escrita por el usuario; vacío significa tamaño original.
    pub width: String,
    /// Alto escrito por el usuario; vacío significa tamaño original.
    pub height: String,
    /// Calidad de salida (1..=100).
    pub quality: u8,
    /// Formato de destino.
    pub format: OutputFormat,
    /// Campo enfocado durante la configuración.
    pub focus: OptimizeField,
    /// Progreso de la ejecución en curso.
    pub progress: OptimizeProgress,
    /// Resumen de la última optimización (completa o cancelada a medias).
    pub results: Option<OptimizeSummary>,
    /// Verdadero si la última ejecución se canceló antes de terminar.
    pub cancelled: bool,
    /// Mensaje de error de la última optimización fallida.
    pub error: Option<String>,
}

impl Default for OptimizeState {
    fn default() -> Self {
        Self {
            step: OptimizeStep::default(),
            input: String::new(),
            width: String::new(),
            height: String::new(),
            quality: 80,
            format: OutputFormat::default(),
            focus: OptimizeField::default(),
            progress: OptimizeProgress::default(),
            results: None,
            cancelled: false,
            error: None,
        }
    }
}
