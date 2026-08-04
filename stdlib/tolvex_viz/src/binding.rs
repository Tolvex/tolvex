use tolvex_rt::stream::RingBuffer;

use crate::chart::{ChartSeries, DataPoint, InteractiveLineChart};

/// A bounded, push-based buffer a chart series can bind to for live updates:
/// `push` appends the newest value and evicts the oldest once the buffer is
/// full, so a chart can re-render from the current snapshot on every tick
/// without re-deriving an ever-growing history. Wraps `tolvex_rt`'s
/// `RingBuffer`, which already implements this eviction policy.
#[derive(Debug, Clone)]
pub struct DataBinding {
    buffer: RingBuffer<f64>,
}

impl DataBinding {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: RingBuffer::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, value: f64) {
        self.buffer.push(value);
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn snapshot(&self) -> Vec<f64> {
        self.buffer.iter().copied().collect()
    }

    /// Materializes the current buffer into a chart series with points
    /// indexed 0..len, suitable for feeding directly into a chart.
    pub fn to_series(&self, name: impl Into<String>) -> ChartSeries {
        let mut series = ChartSeries::new(name);
        series.points.reserve(self.buffer.len());
        for (i, &v) in self.buffer.iter().enumerate() {
            series.push(DataPoint::new(i as f64, v));
        }
        series
    }
}

/// Rebinds the named series inside `chart` from `binding`'s current snapshot,
/// replacing the prior series with that name (or appending if absent). This
/// is the update step a real-time dashboard calls on every new reading.
pub fn bind_series(chart: &mut InteractiveLineChart, binding: &DataBinding, name: &str) {
    let updated = binding.to_series(name);
    if let Some(existing) = chart.series.iter_mut().find(|s| s.name == name) {
        *existing = updated;
    } else {
        chart.add_series(updated);
    }
}
