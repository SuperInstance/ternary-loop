# ternary-loop

Periodicity detection and loop manipulation for ternary signals {-1, 0, +1}. Implements smallest-period finding, bounded loop-length search, crossfade-based seamless looping, and time-stretch with loop preservation — operations essential for audio processing, signal analysis, and ternary pattern compression.

## Why It Matters

Many ternary signals are periodic: oscillating Ising spin configurations, repeating Game of Life patterns, cyclic ternary inference outputs, and ternary-encoded audio. Detecting and manipulating the period of these signals enables:

- **Compression**: A signal of length N with period p stores as O(p) instead of O(N) — a N/p compression ratio
- **Seamless looping**: Audio and pattern synthesis requires smooth loop boundaries (no clicks/jumps)
- **Tempo manipulation**: Time-stretching while preserving the period gives variable-speed playback
- **Pattern classification**: The period is a compact fingerprint for pattern identification

## How It Works

### Smallest Period Detection

The period of a signal s[0..N] is the smallest p such that:

```
∀i ∈ [0, N):   s[i] = s[i mod p]
```

**Algorithm**: For each candidate period p from 1 to N/2, verify all positions match:

```
for p in 1..=N/2:
    if N mod p ≠ 0: continue     // period must divide N
    match = true
    for i in p..N:
        if s[i] ≠ s[i mod p]:
            match = false; break
    if match: return p
return None
```

**Optimization**: Only check periods that divide N. This reduces the search from O(N²) to O(d(N) · N) where d(N) is the number of divisors (typically O(N^(1/3))).

### Bounded Loop Length

Sometimes the exact smallest period is less useful than a period within a specific range. `loop_length(signal, min, max)` finds a period p ∈ [min, max]:

```
for p in min..=min(max, N/2):
    if N mod p ≠ 0: continue
    if all positions match: return p
return None
```

This is useful for audio where the "natural" loop length is perceptually determined (e.g., a musical phrase that's 4–8 bars).

### Crossfade

To create seamless loops, the crate blends the endpoints using a linear crossfade:

```
For i in [0, fade_len):
    t = (i + 1) / (fade_len + 1)
    out[i]             = round(s[i] · t + s[N - fade_len + i] · (1 - t))
    out[N - fade_len + i] = round(s[i] · (1 - t) + s[N - fade_len + i] · t)
```

where t is the crossfade position (0 → 1). At t = 0, the output is purely from the tail; at t = 1, purely from the head. The result is a smooth transition that eliminates boundary discontinuities.

For ternary signals {-1, 0, +1}, the blended values are clamped:

```
out[i] = clamp(round(blend), -1, 1)
```

### Time-Stretch with Loop Preservation

Stretches a signal of length N to length L while preserving the loop structure:

```
out[i] = s[ floor(i · N / L) ]   for i in [0, L)
```

This is nearest-neighbor resampling. For loop-preserving stretch, the signal is first tiled to cover the target length, then resampled. The result maintains the periodicity ratio.

### Complexity

| Operation | Time | Space |
|-----------|------|-------|
| `find_loop(signal)` | O(d(N) · N) | O(1) |
| `loop_length(signal, min, max)` | O((max-min) · N) | O(1) |
| `quantize_to_loop(signal, len)` | O(len) | O(len) |
| `crossfade_loop(signal, fade)` | O(N) | O(N) |
| `loop_stretch(signal, target)` | O(target) | O(target) |

Where N = signal length, d(N) = number of divisors of N.

### Mathematical Properties

**Periodicity test correctness**: `find_loop` is guaranteed to find the smallest period if one exists, because it checks candidates in ascending order.

**Crossfade continuity**: The crossfade guarantees boundary continuity:

```
out[fade_len - 1] → out[fade_len]  is continuous
```

because the blend crosses 50% at the midpoint. The jump discontinuity is spread across `fade_len` samples.

## Quick Start

```rust
use ternary_loop::{find_loop, quantize_to_loop, crossfade_loop, loop_stretch};

// Detect period in a ternary signal
let signal: Vec<i8> = vec![1, -1, 0, 1, -1, 0, 1, -1, 0, 1, -1, 0];
let period = find_loop(&signal);
assert_eq!(period, Some(3));

// Extend a short loop to any length
let short = vec![1, -1];
let extended = quantize_to_loop(&short, 8);
// extended = [1, -1, 1, -1, 1, -1, 1, -1]

// Crossfade for seamless looping
let looped = crossfade_loop(&signal, 2);
// First 2 and last 2 samples are now blended smoothly

// Time-stretch
let stretched = loop_stretch(&signal, 24);  // double the length
assert_eq!(stretched.len(), 24);
```

## API

| Function | Description |
|----------|-------------|
| `find_loop(signal) -> Option<usize>` | Find smallest repeating period |
| `loop_length(signal, min, max) -> Option<usize>` | Find period within range |
| `quantize_to_loop(signal, len) -> Vec<i8>` | Extend/trim to exact length |
| `crossfade_loop(signal, fade_len) -> Vec<i8>` | Blend endpoints for smooth loop |
| `loop_stretch(signal, target_len) -> Vec<i8>` | Resize preserving loop structure |

## Architecture Notes

This crate implements **η (eta) layer** signal processing in the γ + η = C framework:

- **η (eta)**: The analysis engine — period detection, crossfade computation, time-stretching. This crate provides η-layer loop operations on ternary signals.
- **γ (gamma)**: External scheduling — when to apply loop operations, how to chain them in a processing pipeline. Provided by ecosystem coordination crates.
- **C**: The complete signal processing system. The ternary domain {-1, 0, +1} is shared with Ising spins, Life cells, and ternary weights, enabling cross-domain pattern analysis (e.g., detecting periodic behavior in Ising simulations or Life oscillators).

## References

- **Periodicity Detection**: Knuth, D.E., "The Art of Computer Programming, Volume 2: Seminumerical Algorithms," Section 4.6.2 on finding periods, Addison-Wesley, 1997.
- **String Periodicity**: Crochemore, M., "An Optimal Algorithm for Computing the Repetitions in a Word," Information Processing Letters, 12(5), 244-250, 1981.
- **Crossfade Techniques**: Zölzer, U., "DAFX: Digital Audio Effects," Wiley, 2011. Chapter 6 on time-stretching and pitch-shifting.
- **Signal Resampling**: Smith, J.O., "Digital Audio Resampling," 2013. Online: https://ccrma.stanford.edu/~jos/resample/
- **Loop Detection in CA**: Jen, E., "Aperiodicity in One-Dimensional Cellular Automata," Physica D, 45(1-3), 3-18, 1990.

## License

MIT
