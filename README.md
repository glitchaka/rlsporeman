# rlsporeman

Demo mínimo para comprobar si la hoja de sprites generada funciona realmente como un asset animado.

La criatura usa directamente un spritesheet de 6 cuadros y camina sobre una superficie. El render usa filtro `Nearest` para no suavizar el pixel art.

## Compilar

Necesitas Rust estable y Cargo.

```bash
git clone https://github.com/glitchaka/rlsporeman.git
cd rlsporeman
cargo run --release
```

El primer build descargará `macroquad` y sus dependencias.

## Controles

- `A` / `←`: mover a la izquierda
- `D` / `→`: mover a la derecha
- `Espacio`: pausar/reanudar
- Sin pulsar teclas, la criatura continúa caminando automáticamente y rebota en los extremos de la ventana.

## Asset

`assets/blippik_walk_spritesheet.png`

- 6 cuadros horizontales
- celda: 160 × 180 px
- fondo transparente
- el demo selecciona cada cuadro mediante un rectángulo de origen en la textura

La finalidad de este proyecto es detectar de forma visible si existen saltos de anatomía, escala, alineación o volumen entre cuadros. No pretende ocultar esos defectos con interpolación.
