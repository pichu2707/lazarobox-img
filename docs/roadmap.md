# Roadmap

## Filosofía

LazaroBox Image Optimizer se desarrollará de forma incremental.

Cada versión debe aportar una mejora completa y estable, manteniendo siempre una arquitectura limpia y una documentación actualizada.

---

# v0.1 — MVP

## Objetivo

Construir un optimizador funcional.

### Estado

- [x] Scanner de imágenes
- [x] Inspector
- [x] Optimizador
- [x] Redimensionado
- [x] Conversión WebP
- [x] Calidad configurable
- [x] Exportación
- [x] Resumen básico

---

# v0.2 — Motor de decisiones

## Objetivo

Separar completamente las decisiones del proceso de optimización.

### Pendiente

- [ ] Policy Engine
- [ ] Motivos de exclusión
- [ ] Estadísticas de imágenes omitidas
- [ ] Resumen avanzado

---

# v0.3 — Metadatos

## Objetivo

Gestionar correctamente la información adicional de las imágenes.

### Pendiente

- [ ] Lectura EXIF
- [ ] Eliminación de GPS
- [ ] Conservación opcional de metadatos
- [ ] Perfiles de color
- [ ] Copyright
- [ ] Orientación automática

---

# v0.4 — Nuevos formatos

## Objetivo

Convertir LazaroBox en un conversor multi-formato.

### Pendiente

- [ ] AVIF
- [ ] JPEG
- [ ] PNG
- [ ] GIF (si aplica)
- [ ] JPEG XL (evaluación)

---

# v0.5 — Rendimiento

## Objetivo

Reducir tiempos de procesamiento.

### Pendiente

- [ ] Procesamiento paralelo
- [ ] Benchmarks
- [ ] Caché de inspección
- [ ] Optimización de memoria

---

# v0.6 — Interfaz

## Objetivo

Mejorar la experiencia de uso.

### Pendiente

- [ ] Barra de progreso
- [ ] Ratatui
- [ ] Colores
- [ ] Estadísticas en tiempo real

---

# v1.0

## Objetivo

Primera versión estable.

### Requisitos

- Arquitectura estable.
- Documentación completa.
- Tests.
- Benchmarks.
- CLI profesional.
- Publicación en crates.io.
- Releases en GitHub.

# LazaroBox Image v0.4

## Objetivo

Completar el sistema de gestión de metadatos permitiendo leer, modificar, eliminar y escribir información de forma segura sobre las imágenes.

La v0.4 convierte el motor de metadatos en una herramienta completa para fotógrafos, desarrolladores, webmasters, profesionales SEO y creadores de contenido.

---

# Nuevas funcionalidades

## Metadata Writer

Implementar el sistema de escritura de metadatos.

Capacidades:

- escribir nuevos metadatos
- modificar metadatos existentes
- eliminar metadatos
- preservar metadatos existentes
- sobrescribir únicamente los campos indicados

---

## Metadata Profiles

Crear perfiles predefinidos para distintos casos de uso.

Ejemplos:

- SEO
- SEO Local
- Open Graph
- WordPress
- Shopify
- Redes Sociales
- Fotografía
- IA
- Limpieza completa

Cada perfil aplicará automáticamente un conjunto de reglas sobre los metadatos.

---

## Metadata Diff

Comparar:

- imagen original
- imagen optimizada

Mostrando:

- campos añadidos
- campos eliminados
- campos modificados
- campos preservados

---

## GPS

Lectura y escritura de:

- Latitud
- Longitud
- Altitud

Permitiendo:

- mantener
- modificar
- eliminar

---

## Derechos

Soporte para:

- Author
- Copyright
- License
- Source URL

---

## IA

Detectar software conocido:

- Midjourney
- Stable Diffusion
- ComfyUI
- FLUX
- Fooocus
- DALL·E

Permitir:

- mantener información
- eliminar trazas IA
- añadir información personalizada

---

## Metadata Report

Nuevo informe específico de metadatos.

Ejemplo:

WEB

✓ Description

✓ Alt

✗ Keywords

SEO

✓ GPS

✗ Business

RIGHTS

✓ Author

✓ Copyright

AI

✓ Software

✗ Model

TECHNICAL

✓ ICC

✓ Orientation

---

## Batch Metadata

Aplicar modificaciones a múltiples imágenes simultáneamente.

---

## Tests

Añadir batería de imágenes de prueba:

- sin EXIF
- EXIF completo
- GPS
- Photoshop
- Lightroom
- IA
- WordPress
- ICC

---

# Arquitectura

La arquitectura definida en la v0.2 no cambia.

Se añaden:

metadata/

reader/

writer/

profiles/

diff/

tests/

manteniendo el mismo modelo ImageMetadata.

---

# Objetivo final de la versión

Convertir LazaroBox Image en una herramienta completa de gestión de metadatos preparada para la futura interfaz TUI y para automatización mediante CLI.
