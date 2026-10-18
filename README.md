# rlsporeman

Modular pixel-art creature asset pack and playable creature-builder demo.

## Asset pack

`assets/creature_parts/` contains **33 independent part assets**, grouped by slot:

- 4 bodies
- 5 eye sets
- 5 mouths
- 4 leg sets
- 3 arm sets
- 3 tails
- 3 antennae
- 3 back structures
- 3 skin patterns

Every `.part` file defines the visual parameters for one swappable piece. At startup the demo builds matching **8-direction × 4-frame** pixel-art atlases from those parts and layers them on the same pivot. `manifest.txt` defines the shared asset contract and `preset_blippik.txt` is one example creature.

## Controls

- `WASD` / arrows: move
- `Tab`: select creature-part slot
- `Q` / `E`: previous / next part

## Build

```bash
cargo run --release
```

The old `creature.rle.b64` loader is no longer used. All part definitions are embedded with `include_str!`, so the executable does not depend on its working directory or external asset lookup when launched by double-clicking it.
