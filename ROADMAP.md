# 🗺️ Cargo QC Roadmap

Este documento define las fases de desarrollo para elevar `cargo-qc` a un estándar profesional de `crates.io`. La regla de oro es completar cada hito de una versión antes de pasar a la siguiente.

## v0.4.0: Arquitectura y Bases de UX
- [ ] Refactorización de Arquitectura: Separar el código en `src/lib.rs` (lógica) y `src/bin/main.rs` (CLI).
- [ ] Implementar manejo de errores avanzado utilizando `anyhow` y `thiserror`.
- [ ] Incorporar la librería `clap` para manejar argumentos por línea de comandos (ej. banderas `--skip-fmt`, `--ci`).
- [ ] Añadir soporte oficial para que el pipeline ejecute `cargo test` (junto a fmt, clippy y build).

## v0.5.0: Estética y Experiencia Visual
- [ ] Implementar la librería `indicatif` para añadir *spinners* animados de progreso en la terminal.
- [ ] Asegurar compatibilidad estricta con la variable de entorno `NO_COLOR`.
- [ ] Mejorar los mensajes de error mostrados al usuario para que sean más descriptivos y limpios.

## v0.6.0: Configuración y Testing
- [ ] Soporte para archivo de configuración opcional (`cargo-qc.toml`) para que los usuarios puedan habilitar/deshabilitar checks específicos en sus repositorios de forma persistente.
- [ ] Implementar pruebas de integración (Integration Tests) utilizando la librería `assert_cmd` para simular la ejecución del CLI.
- [ ] Agregar pruebas unitarias (Unit Tests) a la lógica principal en `lib.rs`.

## v0.7.0: Estandarización Open Source
- [ ] Implementar herramientas para generación automatizada de `CHANGELOG.md` (ej. `git-cliff`).
- [ ] Crear el documento `CONTRIBUTING.md` con guías claras para aportar al proyecto.
- [ ] Actualizar el `README.md` añadiendo un GIF demostrativo o un SVG animado (`vhs`) mostrando la herramienta en acción.

## v0.8.0 / v1.0.0: La Versión Especial (Automatización e Integración Continua)
- [ ] Configurar pipelines completos de GitHub Actions para CI.
- [ ] Crear el Action de despliegue automático a `crates.io` al subir un tag semántico.
- [ ] Proveer instrucciones claras y un Github Action de `cargo-qc` para que los usuarios lo consuman directamente en sus propios repositorios.
