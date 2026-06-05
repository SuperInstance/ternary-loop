# ternary-loop

**Period detection and seamless looping. Find the repeat, close the circle, play it forever.**

Every signal that repeats is a loop. The heartbeat of electronic music. The frame buffer of a GIF. The recurring pattern in a cellular automaton. The question is always the same: *where does it loop?* Find the shortest repeating period, trim to that length, and you have a tile that can play forever without a seam.

This crate detects the period of ternary signals, extracts the loop, and provides tools for seamless playback: crossfading the loop boundaries so the splice is invisible, stretching or compressing the loop to different lengths, and layering loops of different periods into polyrhythmic structures.

## What's Inside

- **`find_period(signal)`** — find the shortest repeating period in the signal
- **`extract_loop(signal, period)`** — extract one cycle of the loop
- **`crossfade_loop(signal, period, fade_len)`** — crossfade the end into the beginning for seamless looping
- **`extend_loop(loop_data, repetitions)`** — tile the loop N times
- **`stretch_loop(loop_data, target_length)`** — time-stretch the loop to a new length
- **`is_periodic(signal, period)`** — verify the signal actually repeats at this period
- **`period_histogram(signal)`** — all detected periods and their strengths
- **`quantize_loop(signal, grid)`** — snap the loop length to the nearest grid value

## Quick Example

```rust
use ternary_loop::*;

let signal = vec![1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0];

// Find the period
let period = find_period(&signal);
assert_eq!(period, 4); // [1, 0, -1, 0] repeats

// Extract one loop
let one_loop = extract_loop(&signal, period);
// [1, 0, -1, 0]

// Extend: tile it 10 times
let extended = extend_loop(&one_loop, 10);
// [1, 0, -1, 0, 1, 0, -1, 0, ...] × 10

// Crossfade for seamless looping
let seamless = crossfade_loop(&signal, period, 2);
// Last 2 samples fade into first 2 — no click at the splice
```

## The Deeper Truth

**Period detection is the foundation of rhythm.** A loop is a pattern that repeats. The period is the pattern length. Once you know the period, you know the tempo, you know the rhythm, you know where the downbeat is. Every drum machine, every sequencer, every looper pedal does exactly this: find the period and tile it.

In ternary, period detection has a unique advantage: the signal only has 3 values, so the autocorrelation function (which period detection is built on) is exact and fast. No floating-point noise, no approximation — the period is either there or it isn't. The period-8 cycle from ternary-fib is the most important loop in the fleet: it's the fundamental rhythm that many other patterns reduce to.

The crossfade is the art: a bad splice creates a click (an abrupt transition at the loop boundary). A good crossfade smooths the transition by blending the end of the loop into the beginning. In ternary, the crossfade is particularly interesting because the blending operation is ternary addition (mod 3) — which can create entirely new values at the splice point. The seam becomes a creative feature, not a bug.

**Use cases:**
- **Music production** — loop detection for sampling and beat-making
- **Generative music** — extract loops from generative processes, tile them
- **Signal analysis** — find periodicity in any ternary data
- **Game audio** — loop ambient sounds and music seamlessly
- **Cellular automata** — detect oscillators in CA evolution

## See Also

- **ternary-fib** — period-8 as the fundamental ternary rhythm
- **ternary-rhythm** — rhythm pattern generation (loops are the building blocks)
- **ternary-polyrhythm** — layer loops of different periods
- **ternary-crossfader** — the crossfade curves used at loop boundaries
- **ternary-echo** — echo creates implicit loops (repeating patterns at delay intervals)
- **ternary-wave** — generate the signals you're looping

## Install

```bash
cargo add ternary-loop
```

## License

MIT
