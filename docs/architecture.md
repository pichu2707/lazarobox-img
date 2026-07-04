# Architecture

## Objetivo

LazaroBox Image Optimizer es una herramienta CLI en Rust para optimizar imágenes locales mediante redimensionado, conversión de formato y compresión.

## Módulos

### scanner.rs

Localiza imágenes en una ruta de entrada.

No optimiza, no inspecciona metadatos y no exporta archivos.

### inspector.rs

Lee información básica de una imagen:

- ruta
- nombre
- peso
- anchura
- altura

### optimizer.rs

Orquesta el proceso de optimización:

- carga la imagen
- calcula tamaño destino
- redimensiona
- llama al encoder adecuado
- devuelve un resultado comparable

### encoders/

Contiene la lógica específica de codificación por formato.

Actualmente:

- WebP

### export.rs

Gestiona rutas de salida:

- crea carpeta `optimized-*`
- genera nombres de archivo finales

### report.rs

Muestra resultados por terminal.

Actualmente usa `println!`. En el futuro podrá evolucionar hacia Ratatui.

### policy.rs

Decidirá si una imagen debe optimizarse o saltarse.

### types/

Contiene tipos del dominio, como `OutputFormat`.

## Pipeline

```txt
Input Path
↓
Scanner
↓
Inspector
↓
Policy
↓
Optimizer
↓
Encoders
↓
Export
↓
Report
```
