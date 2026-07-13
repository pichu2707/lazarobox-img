use crate::app::{
    AppState, MenuItem, MetaField, OptimizeField, OptimizeProgress, OptimizeStep, OptimizeSummary,
    Screen,
};
use crate::types::OutputFormat;
use crate::update::UpdateStatus;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Calidad por defecto que usa la pantalla de conversión (1..=100).
const CONVERT_QUALITY: u8 = 80;

pub struct AppController {
    pub state: AppState,
}

impl AppController {
    pub fn new() -> Self {
        Self {
            state: AppState::default(),
        }
    }

    /// Navega a la pantalla indicada y prepara el navegador y los estados de
    /// vista de las pantallas que lo usan.
    pub fn go_to(&mut self, screen: Screen) {
        self.state.screen = screen;
        self.state.browser.refresh();
        self.state.inspect.viewing = false;
        self.state.convert.viewing = false;
        self.state.metadata_view.viewing = false;
        self.state.optimize.step = OptimizeStep::Input;
    }

    // ---------------------------------------------------------------------
    // Ajustes / actualizaciones
    // ---------------------------------------------------------------------

    /// Marca una comprobación de actualización como pendiente.
    pub fn settings_check_updates(&mut self) {
        self.state.update.latest_version = None;
        self.state.update.error = None;
        self.state.update.log = None;
        self.state.update.status = UpdateStatus::Checking;
    }

    /// Comprueba la última versión estable publicada en crates.io.
    pub fn settings_run_update_check(&mut self) {
        if self.state.update.status != UpdateStatus::Checking {
            return;
        }

        match crate::update::check_latest_stable() {
            Ok(result) => {
                self.state.update.latest_version = Some(result.latest_version);
                self.state.update.status = result.status;
                self.state.update.error = None;
            }
            Err(err) => {
                self.state.update.latest_version = None;
                self.state.update.status = UpdateStatus::Error;
                self.state.update.error = Some(err.to_string());
            }
        }
    }

    /// Pide confirmación antes de ejecutar Homebrew.
    pub fn settings_confirm_homebrew_update(&mut self) {
        if self.state.update.status != UpdateStatus::UpdateAvailable {
            return;
        }

        self.state.update.status = UpdateStatus::Confirming;
        self.state.update.error = None;
        self.state.update.log = Some(crate::update::homebrew_update_command_text());
    }

    /// Cancela la confirmación y vuelve al estado de actualización disponible.
    pub fn settings_cancel_homebrew_update(&mut self) {
        if self.state.update.status == UpdateStatus::Confirming {
            self.state.update.status = UpdateStatus::UpdateAvailable;
            self.state.update.log = None;
        }
    }

    /// Marca la actualización por Homebrew como pendiente.
    pub fn settings_start_homebrew_update(&mut self) {
        if self.state.update.status != UpdateStatus::Confirming {
            return;
        }

        self.state.update.status = UpdateStatus::Updating;
        self.state.update.error = None;
        self.state.update.log = Some(crate::update::homebrew_update_command_text());
    }

    /// Ejecuta la actualización por Homebrew.
    pub fn settings_run_homebrew_update(&mut self) {
        if self.state.update.status != UpdateStatus::Updating {
            return;
        }

        match crate::update::run_homebrew_update() {
            Ok(result) => {
                self.state.update.status = UpdateStatus::Updated;
                self.state.update.error = None;
                self.state.update.log = Some(format!(
                    "Fórmula actualizada: {}\n{}\nReinicia la app para usar la nueva versión.",
                    result.formula, result.output
                ));
            }
            Err(err) => {
                let message = err.to_string();
                self.state.update.status = UpdateStatus::Error;
                self.state.update.error = Some(message.clone());
                self.state.update.log = Some(message);
            }
        }
    }

    /// Mueve la selección del menú una posición hacia abajo, con wrap.
    pub fn select_next(&mut self) {
        let count = MenuItem::ALL.len();
        self.state.selected = (self.state.selected + 1) % count;
    }

    /// Mueve la selección del menú una posición hacia arriba, con wrap.
    pub fn select_previous(&mut self) {
        let count = MenuItem::ALL.len();
        self.state.selected = (self.state.selected + count - 1) % count;
    }

    /// Ítem del menú actualmente resaltado en el Home.
    pub fn current_item(&self) -> MenuItem {
        MenuItem::ALL[self.state.selected]
    }

    // ---------------------------------------------------------------------
    // Navegador de archivos (compartido)
    // ---------------------------------------------------------------------

    /// Mueve la selección del navegador hacia arriba.
    pub fn browser_up(&mut self) {
        self.state.browser.move_up();
    }

    /// Mueve la selección del navegador hacia abajo.
    pub fn browser_down(&mut self) {
        self.state.browser.move_down();
    }

    /// Sube a la carpeta padre en el navegador.
    pub fn browser_parent(&mut self) {
        self.state.browser.to_parent();
    }

    /// Datos de la entrada resaltada: `(es_carpeta, ruta)`, o `None` si vacío.
    fn selected_entry(&self) -> Option<(bool, std::path::PathBuf)> {
        self.state
            .browser
            .selected_entry()
            .map(|entry| (entry.is_dir, entry.path.clone()))
    }

    // ---------------------------------------------------------------------
    // Inspección
    // ---------------------------------------------------------------------

    /// Activa la entrada resaltada: entra en la carpeta o marca el archivo
    /// como pendiente de inspeccionar (el trabajo real lo hace `inspect_run_pending`).
    pub fn inspect_activate(&mut self) {
        let Some((is_dir, path)) = self.selected_entry() else {
            return;
        };

        if is_dir {
            self.state.browser.enter_selected();
        } else {
            self.state.inspect.result = None;
            self.state.inspect.error = None;
            self.state.inspect.pending = Some(path);
            self.state.inspect.viewing = true;
        }
    }

    /// Inspecciona la imagen pendiente, si la hay. Se llama tras dibujar el
    /// aviso "Procesando…", para que la operación bloqueante no parezca colgada.
    pub fn inspect_run_pending(&mut self) {
        let Some(path) = self.state.inspect.pending.take() else {
            return;
        };

        match crate::inspector::inspect(&path) {
            Ok(info) => {
                self.state.inspect.result = Some(info);
                self.state.inspect.error = None;
            }
            Err(err) => {
                self.state.inspect.result = None;
                self.state.inspect.error = Some(err.to_string());
            }
        }
    }

    /// Vuelve del resultado al navegador.
    pub fn inspect_back(&mut self) {
        self.state.inspect.viewing = false;
    }

    // ---------------------------------------------------------------------
    // Conversión
    // ---------------------------------------------------------------------

    /// Avanza el formato de destino de la conversión.
    pub fn convert_next_format(&mut self) {
        self.state.convert.format = self.state.convert.format.next();
    }

    /// Retrocede el formato de destino de la conversión.
    pub fn convert_previous_format(&mut self) {
        self.state.convert.format = self.state.convert.format.previous();
    }

    /// Activa la entrada resaltada: entra en la carpeta o marca el archivo
    /// como pendiente de convertir.
    pub fn convert_activate(&mut self) {
        let Some((is_dir, path)) = self.selected_entry() else {
            return;
        };

        if is_dir {
            self.state.browser.enter_selected();
        } else {
            self.state.convert.result = None;
            self.state.convert.error = None;
            self.state.convert.pending = Some(path);
            self.state.convert.viewing = true;
        }
    }

    /// Convierte la imagen pendiente, si la hay.
    pub fn convert_run_pending(&mut self) {
        let Some(path) = self.state.convert.pending.take() else {
            return;
        };

        let format = self.state.convert.format;
        match crate::converter::convert(&path, format, CONVERT_QUALITY) {
            Ok(conversion) => {
                self.state.convert.result = Some(conversion);
                self.state.convert.error = None;
            }
            Err(err) => {
                self.state.convert.result = None;
                self.state.convert.error = Some(err.to_string());
            }
        }
    }

    /// Vuelve del resultado al navegador.
    pub fn convert_back(&mut self) {
        self.state.convert.viewing = false;
    }

    // ---------------------------------------------------------------------
    // Metadatos (solo lectura)
    // ---------------------------------------------------------------------

    /// Activa la entrada resaltada: entra en la carpeta o marca el archivo
    /// como pendiente de leer.
    pub fn metadata_activate(&mut self) {
        let Some((is_dir, path)) = self.selected_entry() else {
            return;
        };

        if is_dir {
            self.state.browser.enter_selected();
        } else {
            self.state.metadata_view.result = None;
            self.state.metadata_view.error = None;
            self.state.metadata_view.pending = Some(path);
            self.state.metadata_view.viewing = true;
        }
    }

    /// Lee los metadatos de la imagen pendiente, si la hay.
    pub fn metadata_run_pending(&mut self) {
        let Some(path) = self.state.metadata_view.pending.take() else {
            return;
        };

        match crate::metadata::read_metadata(&path) {
            Ok(metadata) => {
                self.state.metadata_view.result = Some(metadata);
                self.state.metadata_view.error = None;
            }
            Err(err) => {
                self.state.metadata_view.result = None;
                self.state.metadata_view.error = Some(err.to_string());
            }
        }

        self.state.metadata_view.path = Some(path);
        self.state.metadata_view.saved = None;
    }

    /// Entra en el formulario de edición, precargando el alt text y GPS actuales.
    pub fn metadata_start_edit(&mut self) {
        if self.state.metadata_view.path.is_none() {
            return;
        }

        let view = &mut self.state.metadata_view;
        let gps = view
            .result
            .as_ref()
            .and_then(|m| Some((m.seo.latitude.value?, m.seo.longitude.value?)));

        view.alt_input = view
            .result
            .as_ref()
            .and_then(|m| {
                m.web
                    .alt_text
                    .value
                    .clone()
                    .or_else(|| m.web.description.value.clone())
            })
            .unwrap_or_default();
        if let Some((lat, lon)) = gps {
            view.lat_input = lat.to_string();
            view.lon_input = lon.to_string();
        } else {
            view.lat_input.clear();
            view.lon_input.clear();
        }
        view.remove_gps = false;
        view.remove_ai = false;
        view.focus = MetaField::AltText;
        view.error = None;
        view.saved = None;
        view.editing = true;
    }

    /// Sale del formulario de edición sin guardar.
    pub fn metadata_cancel_edit(&mut self) {
        self.state.metadata_view.editing = false;
    }

    /// Enfoca el siguiente campo del formulario, con wrap.
    pub fn metadata_focus_next(&mut self) {
        self.state.metadata_view.focus = next_meta_field(self.state.metadata_view.focus);
    }

    /// Enfoca el campo anterior del formulario, con wrap.
    pub fn metadata_focus_previous(&mut self) {
        // Cuatro avances equivalen a un retroceso en un ciclo de seis.
        for _ in 0..5 {
            self.state.metadata_view.focus = next_meta_field(self.state.metadata_view.focus);
        }
    }

    /// Escribe un carácter en el campo de texto enfocado.
    pub fn metadata_edit_char(&mut self, c: char) {
        let view = &mut self.state.metadata_view;
        match view.focus {
            MetaField::AltText => view.alt_input.push(c),
            MetaField::Lat if is_coord_char(c) => view.lat_input.push(c),
            MetaField::Lon if is_coord_char(c) => view.lon_input.push(c),
            _ => {}
        }
    }

    /// Borra un carácter del campo de texto enfocado.
    pub fn metadata_edit_backspace(&mut self) {
        let view = &mut self.state.metadata_view;
        match view.focus {
            MetaField::AltText => {
                view.alt_input.pop();
            }
            MetaField::Lat => {
                view.lat_input.pop();
            }
            MetaField::Lon => {
                view.lon_input.pop();
            }
            _ => {}
        }
    }

    /// Alterna el campo booleano enfocado (quitar GPS / quitar IA).
    pub fn metadata_toggle(&mut self) {
        let view = &mut self.state.metadata_view;
        match view.focus {
            MetaField::RemoveGps => view.remove_gps = !view.remove_gps,
            MetaField::RemoveAi => view.remove_ai = !view.remove_ai,
            _ => {}
        }
    }

    /// Guarda los cambios en disco. En caso de éxito relee los metadatos.
    pub fn metadata_save(&mut self) {
        let Some(path) = self.state.metadata_view.path.clone() else {
            return;
        };

        let view = &self.state.metadata_view;
        let gps = match (parse_coord(&view.lat_input), parse_coord(&view.lon_input)) {
            (Some(lat), Some(lon)) => Some((lat, lon)),
            _ => None,
        };

        let edit = crate::metadata::editor::MetadataEdit {
            alt_text: Some(view.alt_input.clone()),
            gps,
            remove_gps: view.remove_gps,
            remove_ai: view.remove_ai,
        };

        match crate::metadata::editor::apply(&path, &edit) {
            Ok(()) => {
                self.state.metadata_view.error = None;
                self.state.metadata_view.saved = Some("✔ Metadatos guardados".to_string());
                self.state.metadata_view.editing = false;
                // Relee para reflejar los cambios.
                if let Ok(metadata) = crate::metadata::read_metadata(&path) {
                    self.state.metadata_view.result = Some(metadata);
                }
            }
            Err(err) => {
                self.state.metadata_view.saved = None;
                self.state.metadata_view.error = Some(err.to_string());
            }
        }
    }

    /// Vuelve del resultado al navegador.
    pub fn metadata_back(&mut self) {
        self.state.metadata_view.viewing = false;
    }

    // ---------------------------------------------------------------------
    // Optimización (wizard: Input -> Config -> Results)
    // ---------------------------------------------------------------------

    /// Activa la entrada resaltada: entra en la carpeta o elige el archivo
    /// como entrada y pasa a configurar.
    pub fn optimize_activate(&mut self) {
        let Some((is_dir, path)) = self.selected_entry() else {
            return;
        };

        if is_dir {
            self.state.browser.enter_selected();
        } else {
            self.state.optimize.input = path.display().to_string();
            self.state.optimize.step = OptimizeStep::Config;
        }
    }

    /// Usa la carpeta actual del navegador como entrada y pasa a configurar.
    pub fn optimize_use_current_dir(&mut self) {
        self.state.optimize.input = self.state.browser.current_dir.display().to_string();
        self.state.optimize.step = OptimizeStep::Config;
    }

    /// Vuelve del paso de configuración a la selección de entrada.
    pub fn optimize_back_to_input(&mut self) {
        self.state.optimize.step = OptimizeStep::Input;
    }

    /// Vuelve del paso de resultados al de configuración.
    pub fn optimize_back_to_config(&mut self) {
        self.state.optimize.step = OptimizeStep::Config;
    }

    /// Enfoca el siguiente campo de configuración, con wrap.
    pub fn optimize_focus_next(&mut self) {
        self.state.optimize.focus = match self.state.optimize.focus {
            OptimizeField::Width => OptimizeField::Height,
            OptimizeField::Height => OptimizeField::Quality,
            OptimizeField::Quality => OptimizeField::Format,
            OptimizeField::Format => OptimizeField::Width,
        };
    }

    /// Enfoca el campo de configuración anterior, con wrap.
    pub fn optimize_focus_previous(&mut self) {
        self.state.optimize.focus = match self.state.optimize.focus {
            OptimizeField::Width => OptimizeField::Format,
            OptimizeField::Height => OptimizeField::Width,
            OptimizeField::Quality => OptimizeField::Height,
            OptimizeField::Format => OptimizeField::Quality,
        };
    }

    /// Ajusta el campo enfocado. `delta` es +1 (derecha) o -1 (izquierda).
    /// Las dimensiones también se pueden escribir con dígitos.
    pub fn optimize_adjust(&mut self, delta: i32) {
        match self.state.optimize.focus {
            OptimizeField::Width => adjust_dimension(&mut self.state.optimize.width, delta),
            OptimizeField::Height => adjust_dimension(&mut self.state.optimize.height, delta),
            OptimizeField::Quality => {
                let quality = self.state.optimize.quality as i32 + delta * 5;
                self.state.optimize.quality = quality.clamp(1, 100) as u8;
            }
            OptimizeField::Format => {
                self.state.optimize.format = if delta >= 0 {
                    self.state.optimize.format.next()
                } else {
                    self.state.optimize.format.previous()
                };
            }
        }
    }

    /// Escribe un dígito en la dimensión enfocada (anchura o alto).
    pub fn optimize_config_char(&mut self, c: char) {
        if !c.is_ascii_digit() {
            return;
        }

        let field = match self.state.optimize.focus {
            OptimizeField::Width => &mut self.state.optimize.width,
            OptimizeField::Height => &mut self.state.optimize.height,
            _ => return,
        };

        if field.len() < 5 {
            field.push(c);
        }
    }

    /// Borra un dígito de la dimensión enfocada (anchura o alto).
    pub fn optimize_config_backspace(&mut self) {
        match self.state.optimize.focus {
            OptimizeField::Width => {
                self.state.optimize.width.pop();
            }
            OptimizeField::Height => {
                self.state.optimize.height.pop();
            }
            _ => {}
        }
    }

    /// Prepara la ejecución: escanea las imágenes y pasa al paso `Running`.
    /// Si algo falla, va a resultados con el error. No hace nada sin entrada.
    pub fn optimize_start_run(&mut self) {
        let input = self.state.optimize.input.trim().to_string();
        if input.is_empty() {
            return;
        }

        match prepare_run(Path::new(&input)) {
            Ok((images, output_dir)) => {
                let total = images.len();
                self.state.optimize.progress = OptimizeProgress {
                    queue: images,
                    done: 0,
                    total,
                    current_file: String::new(),
                    acc_original: 0,
                    acc_optimized: 0,
                    output_dir,
                };
                self.state.optimize.results = None;
                self.state.optimize.cancelled = false;
                self.state.optimize.error = None;
                self.state.optimize.step = OptimizeStep::Running;
            }
            Err(err) => {
                self.state.optimize.results = None;
                self.state.optimize.error = Some(err.to_string());
                self.state.optimize.step = OptimizeStep::Results;
            }
        }
    }

    /// Cancela la ejecución en curso: conserva lo ya procesado y muestra el
    /// resultado marcado como cancelado.
    pub fn optimize_cancel(&mut self) {
        self.state.optimize.progress.queue.clear();
        self.finish_optimization();
        self.state.optimize.cancelled = true;
    }

    /// Procesa una imagen de la cola (o finaliza si está vacía). Pensado para
    /// llamarse una vez por fotograma mientras el paso es `Running`, de modo
    /// que la barra de progreso avance de forma visible.
    pub fn optimize_tick(&mut self) {
        let Some(image) = self.next_queued_image() else {
            self.finish_optimization();
            return;
        };

        self.state.optimize.progress.current_file = image
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default();

        let width = parse_dimension(&self.state.optimize.width);
        let height = parse_dimension(&self.state.optimize.height);
        let quality = self.state.optimize.quality;
        let format = self.state.optimize.format;
        let output_dir = self.state.optimize.progress.output_dir.clone();

        match process_one(&image, &output_dir, width, height, quality, format) {
            Ok((original, optimized)) => {
                self.state.optimize.progress.acc_original += original;
                self.state.optimize.progress.acc_optimized += optimized;
                self.state.optimize.progress.done += 1;
            }
            Err(err) => {
                self.state.optimize.results = None;
                self.state.optimize.error = Some(err.to_string());
                self.state.optimize.step = OptimizeStep::Results;
            }
        }
    }

    /// Saca la siguiente imagen de la cola, si queda alguna.
    fn next_queued_image(&mut self) -> Option<std::path::PathBuf> {
        if self.state.optimize.progress.queue.is_empty() {
            None
        } else {
            Some(self.state.optimize.progress.queue.remove(0))
        }
    }

    /// Construye el resumen final y pasa a resultados.
    fn finish_optimization(&mut self) {
        let progress = &self.state.optimize.progress;
        self.state.optimize.results = Some(OptimizeSummary {
            count: progress.done,
            original_total: progress.acc_original,
            optimized_total: progress.acc_optimized,
            output_dir: progress.output_dir.clone(),
        });
        self.state.optimize.error = None;
        self.state.optimize.step = OptimizeStep::Results;
    }
}

/// Siguiente campo del formulario de metadatos, con wrap.
fn next_meta_field(field: MetaField) -> MetaField {
    match field {
        MetaField::AltText => MetaField::Lat,
        MetaField::Lat => MetaField::Lon,
        MetaField::Lon => MetaField::RemoveGps,
        MetaField::RemoveGps => MetaField::RemoveAi,
        MetaField::RemoveAi => MetaField::Save,
        MetaField::Save => MetaField::AltText,
    }
}

/// Caracteres válidos al escribir una coordenada (dígitos, signo y punto).
fn is_coord_char(c: char) -> bool {
    c.is_ascii_digit() || c == '.' || c == '-'
}

/// Interpreta una coordenada escrita en grados decimales.
fn parse_coord(raw: &str) -> Option<f64> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<f64>().ok()
}

/// Interpreta una dimensión escrita: vacío o inválido = tamaño original.
fn parse_dimension(raw: &str) -> Option<u32> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    trimmed.parse::<u32>().ok().filter(|&value| value > 0)
}

/// Ajusta una dimensión escrita con `± delta * 10`; 0 vuelve a vacío (auto).
fn adjust_dimension(field: &mut String, delta: i32) {
    let current: i32 = field.trim().parse().unwrap_or(0);
    let next = (current + delta * 10).max(0);

    if next == 0 {
        field.clear();
    } else {
        *field = next.to_string();
    }
}

/// Escanea las imágenes de la entrada y crea la carpeta de salida.
fn prepare_run(input: &Path) -> Result<(Vec<PathBuf>, PathBuf)> {
    let images = crate::scanner::find_images(input)?;
    let output_dir = crate::export::create_output_dir(input)?;
    Ok((images, output_dir))
}

/// Optimiza una imagen y devuelve `(tamaño_original, tamaño_optimizado)`.
fn process_one(
    image: &Path,
    output_dir: &Path,
    width: Option<u32>,
    height: Option<u32>,
    quality: u8,
    format: OutputFormat,
) -> Result<(u64, u64)> {
    let output_file = crate::export::create_output_file(output_dir, image, format.extension())?;
    let result = crate::optimizer::optimize(image, &output_file, width, height, quality, format)?;

    let optimized = result.outputs.iter().map(|output| output.size).sum();
    Ok((result.original.size, optimized))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::ImageMetadata;

    #[test]
    fn metadata_start_edit_preloads_existing_gps() {
        let mut controller = AppController::new();
        let mut metadata = ImageMetadata::default();
        metadata.seo.latitude.value = Some(40.416775);
        metadata.seo.longitude.value = Some(-3.70379);

        controller.state.metadata_view.path = Some(PathBuf::from("image.jpg"));
        controller.state.metadata_view.result = Some(metadata);

        controller.metadata_start_edit();

        assert_eq!(controller.state.metadata_view.lat_input, "40.416775");
        assert_eq!(controller.state.metadata_view.lon_input, "-3.70379");
        assert!(controller.state.metadata_view.editing);
    }

    #[test]
    fn metadata_start_edit_clears_gps_inputs_when_coordinates_are_incomplete() {
        let mut controller = AppController::new();
        let mut metadata = ImageMetadata::default();
        metadata.seo.latitude.value = Some(40.416775);

        controller.state.metadata_view.path = Some(PathBuf::from("image.jpg"));
        controller.state.metadata_view.result = Some(metadata);
        controller.state.metadata_view.lat_input = "stale latitude".to_string();
        controller.state.metadata_view.lon_input = "stale longitude".to_string();

        controller.metadata_start_edit();

        assert!(controller.state.metadata_view.lat_input.is_empty());
        assert!(controller.state.metadata_view.lon_input.is_empty());
    }

    #[test]
    fn settings_homebrew_update_confirmation_can_be_cancelled() {
        let mut controller = AppController::new();
        controller.state.update.status = UpdateStatus::UpdateAvailable;

        controller.settings_confirm_homebrew_update();

        assert_eq!(controller.state.update.status, UpdateStatus::Confirming);
        assert_eq!(
            controller.state.update.log.as_deref(),
            Some("brew update && brew upgrade pichu2707/tap/lazarobox-img")
        );

        controller.settings_cancel_homebrew_update();

        assert_eq!(
            controller.state.update.status,
            UpdateStatus::UpdateAvailable
        );
        assert!(controller.state.update.log.is_none());
    }

    #[test]
    fn settings_homebrew_update_starts_only_after_confirmation() {
        let mut controller = AppController::new();

        controller.settings_start_homebrew_update();
        assert_eq!(controller.state.update.status, UpdateStatus::NotChecked);

        controller.state.update.status = UpdateStatus::Confirming;
        controller.settings_start_homebrew_update();

        assert_eq!(controller.state.update.status, UpdateStatus::Updating);
        assert_eq!(
            controller.state.update.log.as_deref(),
            Some("brew update && brew upgrade pichu2707/tap/lazarobox-img")
        );
    }
}
