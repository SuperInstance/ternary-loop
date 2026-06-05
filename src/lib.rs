#![forbid(unsafe_code)]

/// Find the smallest repeating period in the signal.
pub fn find_loop(signal: &[i8]) -> Option<usize> {
    if signal.is_empty() {
        return None;
    }
    let n = signal.len();
    for period in 1..=n / 2 {
        if n % period != 0 {
            continue;
        }
        let mut ok = true;
        for i in period..n {
            if signal[i] != signal[i % period] {
                ok = false;
                break;
            }
        }
        if ok {
            return Some(period);
        }
    }
    None
}

/// Find a loop length between min and max (inclusive).
pub fn loop_length(signal: &[i8], min: usize, max: usize) -> Option<usize> {
    let n = signal.len();
    for period in min..=max.min(n / 2) {
        if n % period != 0 {
            continue;
        }
        let mut ok = true;
        for i in period..n {
            if signal[i] != signal[i % period] {
                ok = false;
                break;
            }
        }
        if ok {
            return Some(period);
        }
    }
    None
}

/// Trim or extend signal to exactly `len` samples, looping as needed.
pub fn quantize_to_loop(signal: &[i8], len: usize) -> Vec<i8> {
    if signal.is_empty() {
        return vec![0; len];
    }
    (0..len).map(|i| signal[i % signal.len()]).collect()
}

/// Crossfade the loop endpoints by `fade_len` samples for smooth looping.
pub fn crossfade_loop(signal: &[i8], fade_len: usize) -> Vec<i8> {
    if signal.len() < fade_len * 2 || fade_len == 0 {
        return signal.to_vec();
    }
    let mut out = signal.to_vec();
    let n = signal.len();
    for i in 0..fade_len {
        let t = (i + 1) as f64 / (fade_len + 1) as f64;
        let blended = (signal[i] as f64 * t + signal[n - fade_len + i] as f64 * (1.0 - t)).round() as i8;
        out[i] = blended.clamp(-1, 1);
        let blend_end = (signal[i] as f64 * (1.0 - t) + signal[n - fade_len + i] as f64 * t).round() as i8;
        out[n - fade_len + i] = blend_end.clamp(-1, 1);
    }
    out
}

/// Time-stretch signal to target length, preserving loop structure.
pub fn loop_stretch(signal: &[i8], target_len: usize) -> Vec<i8> {
    if signal.is_empty() || target_len == 0 {
        return vec![0; target_len];
    }
    (0..target_len)
        .map(|i| {
            let src_idx = (i as f64 * signal.len() as f64 / target_len as f64) as usize;
            signal[src_idx.min(signal.len() - 1)]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_loop_repeating() {
        let sig = [1, -1, 1, -1, 1, -1];
        assert_eq!(find_loop(&sig), Some(2));
    }

    #[test]
    fn find_loop_single_period() {
        let sig = [1, 0, -1, 1, 0, -1];
        assert_eq!(find_loop(&sig), Some(3));
    }

    #[test]
    fn find_loop_no_repeat() {
        let sig = [1, 0, -1, 0, 1, 1];
        assert_eq!(find_loop(&sig), None);
    }

    #[test]
    fn find_loop_empty() {
        assert_eq!(find_loop(&[]), None);
    }

    #[test]
    fn find_loop_single_element() {
        assert_eq!(find_loop(&[1]), None);
    }

    #[test]
    fn loop_length_within_range() {
        let sig = [1, -1, 1, -1, 1, -1];
        assert_eq!(loop_length(&sig, 1, 4), Some(2));
    }

    #[test]
    fn loop_length_out_of_range() {
        let sig = [1, -1, 1, -1, 1, -1];
        assert_eq!(loop_length(&sig, 3, 5), None);
    }

    #[test]
    fn quantize_to_loop_truncate() {
        let sig = [1, 0, -1];
        assert_eq!(quantize_to_loop(&sig, 2), vec![1, 0]);
    }

    #[test]
    fn quantize_to_loop_extend() {
        let sig = [1, -1];
        assert_eq!(quantize_to_loop(&sig, 6), vec![1, -1, 1, -1, 1, -1]);
    }

    #[test]
    fn quantize_to_loop_empty_signal() {
        assert_eq!(quantize_to_loop(&[], 3), vec![0, 0, 0]);
    }

    #[test]
    fn crossfade_preserves_length() {
        let sig = vec![1, 1, 1, 1, -1, -1];
        let out = crossfade_loop(&sig, 2);
        assert_eq!(out.len(), sig.len());
    }

    #[test]
    fn crossfade_zero_fade() {
        let sig = vec![1, -1, 1, -1];
        let out = crossfade_loop(&sig, 0);
        assert_eq!(out, sig);
    }

    #[test]
    fn crossfade_short_signal() {
        let sig = vec![1, -1];
        let out = crossfade_loop(&sig, 2);
        assert_eq!(out, sig);
    }

    #[test]
    fn loop_stretch_expand() {
        let sig = [1, -1];
        let out = loop_stretch(&sig, 6);
        assert_eq!(out.len(), 6);
        assert!(out.iter().all(|&v| (-1..=1).contains(&v)));
    }

    #[test]
    fn loop_stretch_shrink() {
        let sig = [1, 0, -1, 0, 1, 0];
        let out = loop_stretch(&sig, 3);
        assert_eq!(out.len(), 3);
    }

    #[test]
    fn loop_stretch_empty() {
        let out = loop_stretch(&[], 5);
        assert_eq!(out, vec![0, 0, 0, 0, 0]);
    }
}
