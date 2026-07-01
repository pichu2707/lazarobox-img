# LazaroBox Image Optimizer - Optimization Policy

## Objetivo

Este documento define las reglas que sigue LazaroBox Image Optimizer para decidir cuándo una imagen debe optimizarse, cuándo debe omitirse y qué criterios debe seguir cada optimización.

El objetivo es que todas las decisiones del optimizador sean predecibles, consistentes y fácilmente ampliables sin modificar la arquitectura del proyecto.

---

# Filosofía

LazaroBox no intenta convertir todas las imágenes.

Solo optimiza aquellas en las que exista una mejora real.

Si una imagen ya cumple los requisitos solicitados por el usuario, el optimizador debe evitar trabajo innecesario.

Esta filosofía permite:

- Reducir tiempo de procesamiento.
- Evitar recomprimir imágenes innecesariamente.
- Mantener la máxima calidad posible.
- Reducir el consumo de CPU y memoria.

---

# Pipeline de optimización

Cada imagen seguirá el siguiente flujo.

```
Scanner
    ↓
Inspector
    ↓
Policy
    ↓
Optimizer
    ↓
Export
    ↓
Report
```

Cada módulo tiene una única responsabilidad.

## Scanner

Responsable de localizar las imágenes válidas dentro de una ruta.

Nunca modifica imágenes.

---

## Inspector

Obtiene información de una imagen.

Ejemplo:

- tamaño
- resolución
- formato
- nombre
- ruta

Nunca modifica imágenes.

---

## Policy

Decide si una imagen debe optimizarse.

Nunca modifica imágenes.

Únicamente responde si la imagen debe continuar el pipeline o no.

---

## Optimizer

Realiza las modificaciones necesarias sobre una imagen.

Ejemplos:

- redimensionado
- cambio de formato
- compresión
- calidad

Nunca decide si una imagen debe optimizarse.

---

## Export

Genera las rutas de salida y escribe los archivos optimizados.

No modifica los píxeles.

---

## Report

Presenta al usuario los resultados obtenidos.

No realiza ninguna optimización.

---

# Reglas actuales

Actualmente LazaroBox considera una imagen optimizable cuando al menos una de estas condiciones no se cumple.

## Formato

La imagen ya utiliza el formato solicitado.

Ejemplo:

Imagen:

```
foto.webp
```

Parámetros:

```
--format webp
```

Resultado:

Formato correcto.

---

## Resolución

La resolución supera el tamaño máximo solicitado.

Ejemplo:

```
3840x2160
```

Parámetros:

```
--width 1200
```

Resultado:

Debe redimensionarse.

---

## Calidad

La calidad almacenada no coincide con la solicitada.

Actualmente esta comprobación todavía no está implementada.

Será utilizada para evitar recomprimir imágenes que ya utilizan la calidad adecuada.

---

# Decisión de optimización

Una imagen únicamente podrá omitirse cuando todas las reglas anteriores se cumplan simultáneamente.

```
Formato correcto
AND
Resolución correcta
AND
Calidad correcta

↓

No optimizar
```

Si cualquiera de ellas falla:

```
↓

Optimizar
```

---

# Reglas futuras

El sistema está diseñado para incorporar nuevas reglas sin modificar la arquitectura principal.

Entre ellas:

- peso máximo
- transparencia
- imágenes animadas
- SVG
- AVIF
- JPEG XL
- perfiles de color
- metadatos EXIF
- eliminación de información innecesaria
- límite mínimo de resolución
- detección de miniaturas
- iconos
- logos

---

# Principios de desarrollo

Toda nueva regla deberá cumplir los siguientes principios.

1. Tener una única responsabilidad.

2. Ser independiente del resto.

3. No modificar imágenes.

4. Ser fácilmente testeable.

5. Poder activarse o desactivarse sin afectar al resto del pipeline.

6. Mantener la filosofía de evitar trabajo innecesario siempre que sea posible.
