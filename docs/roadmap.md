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
