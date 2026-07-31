use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RingBuffer<T> {
    buf: VecDeque<T>,
    cap: usize,
    dropped: usize,
}

impl<T> RingBuffer<T> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buf: VecDeque::with_capacity(cap),
            cap,
            dropped: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.cap == 0 {
            self.dropped += 1;
            return;
        }

        if self.buf.len() == self.cap {
            self.buf.pop_front();
            self.dropped += 1;
        }
        self.buf.push_back(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.buf.pop_front()
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn dropped(&self) -> usize {
        self.dropped
    }
}

pub fn tumbling_windows<T: Clone>(xs: &[T], size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        return Vec::new();
    }
    xs.chunks(size).map(|c| c.to_vec()).collect()
}

pub fn sliding_windows<T>(xs: &[T], size: usize, step: usize) -> Vec<&[T]> {
    if size == 0 || step == 0 || xs.len() < size {
        return Vec::new();
    }

    let mut out = Vec::new();
    let mut start = 0;
    while start + size <= xs.len() {
        out.push(&xs[start..start + size]);
        start += step;
    }
    out
}

/// Groups timestamped events into sessions, splitting whenever the gap
/// between consecutive events exceeds `gap_ms`. Assumes `events` is sorted
/// ascending by timestamp.
pub fn session_windows<T: Clone>(events: &[(u64, T)], gap_ms: u64) -> Vec<Vec<(u64, T)>> {
    if events.is_empty() {
        return Vec::new();
    }

    let mut out: Vec<Vec<(u64, T)>> = Vec::new();
    let mut current: Vec<(u64, T)> = vec![events[0].clone()];

    for pair in events.windows(2) {
        let (prev_ts, _) = &pair[0];
        let (ts, val) = &pair[1];
        if ts.saturating_sub(*prev_ts) > gap_ms {
            out.push(std::mem::take(&mut current));
        }
        current.push((*ts, val.clone()));
    }
    out.push(current);
    out
}

pub fn sliding_window_sum(xs: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || xs.len() < window {
        return Vec::new();
    }

    let mut out = Vec::with_capacity(xs.len() - window + 1);
    let mut sum = 0.0;

    for i in 0..xs.len() {
        sum += xs[i];
        if i >= window {
            sum -= xs[i - window];
        }
        if i + 1 >= window {
            out.push(sum);
        }
    }

    out
}
