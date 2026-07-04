# Architecture Guidelines

## Principios generales

- El código debe ser modular, ampliable y fácil de mantener.
- Cada módulo debe tener una responsabilidad clara.
- El proyecto debe poder crecer sin convertir `main.rs` en un archivo central gigante.
- La arquitectura debe favorecer MVPs entregables y ampliaciones progresivas.

## Visibilidad

Por defecto, todo debe ser privado.

Usar:

- `fn` para helpers internos.
- `pub(super)` para helpers compartidos con el módulo padre.
- `pub(crate)` para funciones internas del crate.
- `pub` solo para API pública real del módulo.

## Responsabilidades

- `main.rs` solo debe orquestar.
- Los módulos de dominio no deben imprimir en terminal.
- La UI es la única capa responsable de mostrar información.
- `optimizer` modifica imágenes.
- `encoders` codifica formatos.
- `metadata` lee/modela metadatos.
- `policy` decide.
- `report/ui` presenta.

## Regla de presentación

No usar `println!` en módulos de negocio.

Permitido en:

- `main.rs` temporalmente durante desarrollo.
- `ui/`
- módulos de reporting.

## Regla de datos

Los datos deben viajar mediante modelos claros:

- `ImageInfo`
- `ImageMetadata`
- `OptimizationResult`
- futuros modelos de análisis.

Evitar pasar listas largas de parámetros cuando un modelo represente mejor el dominio.

## Evolución

Antes de añadir una nueva funcionalidad:

1. Definir dónde vive.
2. Ver si requiere modelo nuevo.
3. Añadir parser/encoder/policy si procede.
4. Añadir salida en UI.
5. Documentar.
6. Probar.
