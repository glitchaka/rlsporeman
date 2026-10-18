# rlsporeman

Demo jugable para comprobar un sprite procedural coherente dentro de un juego real.

## Qué contiene

- personaje controlable;
- estado `IDLE`;
- ciclo `WALK` de 6 cuadros;
- salto;
- gravedad;
- suelo y plataformas;
- colisiones laterales y verticales;
- cámara con seguimiento;
- cambio de dirección;
- render `Nearest` para pixel art.

El sprite se construye dentro del programa como cuadros reales de 64×64 con las mismas proporciones, silueta, marcas y línea de apoyo. Así el repositorio no depende del spritesheet roto anterior.

## Compilar

```bash
git clone https://github.com/glitchaka/rlsporeman.git
cd rlsporeman
cargo run --release
```

## Controles

- `A` / `←`: izquierda
- `D` / `→`: derecha
- `Espacio`, `W` o `↑`: saltar

La demo no mueve al personaje automáticamente: responde al jugador y cambia entre idle, walk y aire según su estado.
