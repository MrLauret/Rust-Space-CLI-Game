# Mision Control

Aplicacion de consola en Rust para gestionar una mision espacial. El proyecto esta en una etapa inicial: ya cuenta con un menu, un modelo basico de nave y una tripulacion inicial; las demas acciones de la mision estan pendientes.

## Requisitos

- Rust y Cargo instalados. El proyecto usa la edicion 2024 de Rust.

## Ejecutar

Desde la carpeta del proyecto:

```bash
cargo run
```

Tambien se puede comprobar que el proyecto compila con:

```bash
cargo check
```

## Estado actual

Al iniciar, la nave tiene combustible, oxigeno, energia e integridad al 100%, escudos operativos y estado general bueno. La tripulacion inicial consta de un medico, un ingeniero y un capitan, todos con salud al 100% y estado activo.

El menu ofrece estas opciones:

- `1`: muestra el estado de la nave.
- `0`: sale del programa.

Las opciones para ver la tripulacion, consultar el destino, continuar la mision, consultar el historial y guardar o cargar una partida aparecen en el menu, pero todavia no estan implementadas.

## Estructura

- `src/main.rs`: punto de entrada; crea el motor de la mision e inicia el menu.
- `src/models/`: modelos de nave, planeta, tripulante y evento.
- `src/services/`: logica de la mision y modulo de datos, aun en desarrollo.
- `src/utils/`: utilidades de consola, como limpiar la pantalla y esperar Enter.
- `data/`, `reports/` y `saves/`: carpetas previstas para datos, informes y partidas guardadas; actualmente estan vacias.

## Desarrollo

El proyecto no tiene dependencias externas declaradas por ahora. Para ejecutar las comprobaciones de Cargo:

```bash
cargo test
```
