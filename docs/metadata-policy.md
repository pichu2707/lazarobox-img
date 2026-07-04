# Metadata Policy

## Objetivo

LazaroBox Image Optimizer debe ser capaz de inspeccionar, modificar, conservar o eliminar los metadatos de una imagen de forma controlada.

Los metadatos forman parte de la información de una imagen, pero no de sus píxeles, por lo que se gestionan mediante un módulo independiente del optimizador.

---

# Filosofía

El tratamiento de metadatos debe seguir cuatro principios:

1. Nunca modificar información sin que el usuario lo solicite.
2. Mantener la privacidad como comportamiento por defecto.
3. Conservar únicamente los datos útiles cuando sea necesario.
4. Mantener una trazabilidad completa de todos los cambios realizados.

---

# Pipeline

```txt
Imagen original
        │
        ▼
Metadata Reader
        │
        ▼
Metadata Policy
        │
        ▼
Metadata Writer
        │
        ▼
Imagen final
```

---

# Capacidades

## Lectura

LazaroBox debe poder leer, entre otros:

- EXIF
- IPTC
- XMP
- Perfil de color ICC
- Orientación
- Fecha de captura
- Cámara
- Objetivo
- ISO
- Velocidad de obturación
- Apertura
- Distancia focal
- GPS (latitud y longitud)
- Copyright
- Autor
- Descripción
- Palabras clave
- Alt Text (cuando exista)

---

## Escritura

LazaroBox debe poder escribir:

- Copyright
- Autor
- Descripción
- Palabras clave
- Alt Text
- Copyright URL
- Información personalizada
- Metadatos generados automáticamente

---

## Eliminación

Debe permitir eliminar de forma selectiva:

- GPS
- Información de cámara
- Datos personales
- Historial de edición
- Todo el bloque EXIF
- Todo el bloque IPTC
- Todo el bloque XMP

---

# Política por defecto

Por defecto:

- Mantener orientación correcta.
- Mantener perfil ICC.
- Eliminar datos sensibles únicamente si el usuario lo solicita.
- No sobrescribir información existente sin autorización.

---

# Resultado esperado

Cada imagen debe devolver un informe indicando:

- Si tenía metadatos.
- Qué tipos de metadatos contenía.
- Qué campos fueron modificados.
- Qué campos fueron eliminados.
- Qué campos fueron añadidos.
- Qué campos permanecieron sin cambios.

---

# Resumen final

El informe global deberá mostrar estadísticas como:

```txt
Metadatos

Imágenes con metadatos ............ 128

Sin metadatos ..................... 54

GPS eliminados .................... 23

Alt Text añadidos ................. 80

Copyright actualizado ............. 34

Metadatos conservados ............. 97
```

---

# Arquitectura

La gestión de metadatos será independiente del optimizador.

```txt
metadata/
│
├── mod.rs
├── reader.rs
├── writer.rs
├── policy.rs
└── types.rs
```

---

# Futuras funcionalidades

- Plantillas de metadatos.
- Inserción automática de Copyright.
- Inserción automática de Alt Text generado por IA.
- Sincronización con DAM.
- Edición masiva de metadatos.
- Exportación de metadatos a CSV o JSON.
- Validación de metadatos para SEO y accesibilidad.
