use crate::{Bars, Ppqn, Ticks};

pub type Top = u16;
pub type Bottom = u16;

/// Represents a musical time signature.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TimeSig {
    pub top: Top,
    pub bottom: Bottom,
}

impl TimeSig {

    /// Return how many beats there are in a bar under this time signature.
    #[inline]
    pub fn beats_per_bar(&self) -> f64 {
        4.0 * self.top as f64 / self.bottom as f64
    }

    /// The number of `Ticks` in a single `Bar` with this `TimeSig`.
    #[inline]
    pub fn ticks_per_bar(&self, ppqn: Ppqn) -> Ticks {
        Bars(1).to_ticks(*self, ppqn)
    }

}
