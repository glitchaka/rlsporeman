# rlsporeman

Demo de criatura pixel-art para un juego tipo **Spore / Creature Stage**, no un plataformas lateral.

## Asset

- criatura alienígena volumétrica legible desde todos los ángulos;
- atlas de **8 direcciones**: S, SE, E, NE, N, NW, W, SW;
- **4 cuadros de marcha por dirección**;
- mismo pivote y escala entre cuadros;
- render `Nearest`;
- el atlas está incluido en `src/creature.rle.b64` como imagen indexada/RLE para mantener el repo completamente textual y compilable.

## Demo

- movimiento libre en 8 direcciones;
- cámara superior/3⁄4 que sigue a la criatura;
- terreno alienígena desplazable;
- rocas/obstáculos con colisión;
- `IDLE` al detenerse y `WALK` al moverse;
- sombra de contacto para leer la criatura sobre el terreno.

## Compilar

```bash
git clone https://github.com/glitchaka/rlsporeman.git
cd rlsporeman
cargo run --release
```

## Controles

- `WASD` o flechas: mover la criatura en 8 direcciones.
