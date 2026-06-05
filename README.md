# ternary-loop

**Period detection, loop quantization, and seamless crossfading for ternary signals.**

Every signal that repeats is a loop. The question is: *where does it loop?* This crate finds the shortest repeating period in a ternary signal, then gives you the tools to manipulate that loop — trim it, extend it, stretch it, crossfade it so it plays seamlessly forever.

The crossfade is the secret sauce. Most loop implementations just snap the end to the beginning, which creates a click. This crate blends the endpoints over a configurable fade length, producing loops that are mathematically smooth even in ternary space — the blended values snap to {-1, 0, +1} at the boundaries.

## What's Inside

- **`find_loop(signal)`** — find the shortest repeating period. Returns `None` if the signal doesn't repeat exactly
- **`loop_length(signal, min, max)`** — search for a period within a specific range
- **`quantize_to_loop(signal, len)`** — trim or extend a signal to exactly `len` samples by looping
- **`crossfade_loop(signal, fade_len)`** — blend the loop endpoints over `fade_len` samples for seamless repetition
- **`loop_stretch(signal, target_len)`** — time-stretch a loop to a different length while preserving structure

## Quick Example

```rust
use ternary_loop::*;

// A signal with period 3
let signal = [1, 0, -1, 1, 0, -1];
assert_eq!(find_loop(&signal), Some(3));

// Quantize to exactly 9 samples (3 loops)
let extended = quantize_to_loop(&signal, 9);
// [1, 0, -1, 1, 0, -1, 1, 0, -1]

// Crossfade for seamless looping
let looped = [1, 1, 1, 1, -1, -1];
let smooth = crossfade_loop(&looped, 2);
// Endpoints blended: no click when this loops back to start

// Stretch a 4-sample loop to 12 samples
let short = [1, -1, 1, -1];
let long = loop_stretch(&short, 12);
assert_eq!(long.len(), 12);
```

## The Deeper Truth

**Exact period detection is O(n²) and worth it.** The algorithm tries every possible period from 1 to n/2, checking if the entire signal is consistent with that period. This is brute force, but for ternary signals — which are short and discrete — it's fast and *exact*. No FFT approximations. No windowing. Either the signal repeats or it doesn't.

The crossfade works because ternary values snap cleanly: when you blend `1` and `-1` with equal weight, you get `0` — a valid ternary value. This means crossfaded ternary loops never produce invalid intermediate states. The blend stays in the alphabet.

**Use cases:**
- **Music production** — find the loop point in ternary drum patterns
- **Animation** — seamless ternary sprite loops
- **Data compression** — if a signal loops, store one period instead of the whole thing
- **Game development** — looping AI behavior patterns
- **Scientific computing** — detect periodicity in ternary time series

## See Also

- **ternary-fib** — period-8 ternary Fibonacci (a loop that found itself)
- **ternary-wave** — generate the loops in the first place
- **ternary-echo** — echoes are loops with decay
- **ternary-epoch** — epoch boundaries are loop breakpoints
- **ternary-phase** — phase alignment between overlapping loops

## Install

```bash
cargo add ternary-loop
```

## License

MIT
