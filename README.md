# Lazarobox-IMG

> Los módulos son responsables del comportamiento. Los datos viajan entre ellos.

## v0.0 — Bootstrap

Primera estructura del proyecto y definición de la arquitectura base.

### Incluye

- Creación del proyecto en Rust.
- Configuración inicial de Cargo.
- Organización de módulos.
- Primeras pruebas con imágenes.
- Def

## v0.1 — Optimization Engine

Primera versión funcional del optimizador.

### Incluye

- Escaneo de imagen o carpeta.
- Detección de imágenes válidas.
- Lectura de información básica:
  - nombre
  - peso
  - resolución
- Redimensionado por `width` y/o `height`.
- Conversión a WebP.
- Calidad configurable.
- Exportación a carpeta `optimized-*`.
- Reporte por imagen.
- Resumen final con ahorro total, tiempo y ruta de salida.

### Objetivo

Validar el pipeline principal:

Scanner → Inspector → Optimizer → Encoder → Export → Report

## v0.2 — Metadata Engine

Versión centrada en modelar y leer metadatos.

### Incluye

- Modelo `ImageMetadata`.
- Campos web:
  - descripción
  - alt text
  - keywords
- Campos de derechos:
  - autor
  - copyright
  - licencia
  - URL fuente
- Campos IA:
  - software
  - modelo
  - licencia
  - source URL
- Campos técnicos:
  - orientación
  - perfil de color
- Reader EXIF inicial.
- Parsers separados:
  - web
  - rights
  - ai
  - seo
  - technical
- Normalización de textos.
- Detección inicial de software relacionado con edición/IA.

### Objetivo

Separar la lectura de metadatos del optimizador y crear un modelo ampliable para EXIF, IPTC, XMP, ICC y metadatos IA.

## v0.3 — Presentation Layer

## v0.3 — Architecture & UI Foundation

Versión centrada en la reorganización del proyecto, la separación por responsabilidades y la creación de una interfaz de consola unificada con el estilo LazaroBox.

### Incluye

#### Arquitectura

- Separación del proyecto en módulos independientes.
- Organización de la carpeta `metadata`.
- Organización de la carpeta `ui`.
- Creación de `theme.rs` como lenguaje visual común.
- Separación entre lógica de negocio y presentación.
- Creación de documentación técnica (`docs/`).

#### Interfaz CLI

- Cabecera unificada de LazaroBox.
- Vista del proyecto.
- Vista de progreso.
- Vista individual de optimización.
- Resumen final.
- Helpers reutilizables para toda la interfaz.

#### Metadata

- Separación entre:
  - Reader
  - Parsers
  - Report

- Normalización de valores.
- Modelo de metadatos ampliable.

#### Calidad del código

- Eliminación de `println!` dispersos.
- Centralización de la salida en `ui`.
- Preparación del proyecto para una futura interfaz Ratatui.

### Objetivo

Transformar el proyecto desde un optimizador funcional hacia una aplicación modular, escalable y preparada para incorporar nuevas funcionalidades sin modificar el núcleo del sistema.

v0.0 Bootstrap
│
▼
v0.1 Image Optimization Engine
│
▼
v0.2 Metadata Engine
│
▼
v0.3 Architecture & UI Foundation
│
▼
v0.4 Metadata Planning & Logical Writer
│
▼
v0.5 Ratatui Interactive Interface
│
▼
v0.6 Physical Metadata Writer
│
▼
v0.7 Advanced Formats & Metadata Profiles
│
▼
v1.0 Stable Release

## v0.4-preview — Remove AI Metadata Plan

- Metadata Plan para eliminar trazas IA.
- Writer lógico en memoria.
- Preview antes/después.
- No modifica aún el archivo físico.

## v0.5 — Ratatui TUI

- Selección de acción.
- Selección de imagen/carpeta.
- Configuración de width, height, quality y format.
- Vista de metadata.
- Vista de plan remove-ai.
- Vista de optimización.
- Resumen final.
