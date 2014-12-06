
pub type Top = u8;
pub type Bottom = u8;

/// Represents a musical time signature.
#[deriving(Show, Clone, Encodable, Decodable, PartialEq, Eq)]
pub struct TimeSig {
    pub top: u16,
    pub bottom: u16,
}

impl TimeSig {
    /// Return how many beats there are in a bar under this time signature.
    #[inline]
    pub fn beats_in_a_bar(&self) -> f64 {
        4.0 * self.top as f64 / self.bottom as f64
    }
}

