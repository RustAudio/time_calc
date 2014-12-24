
pub type Top = u16;
pub type Bottom = u16;

/// Represents a musical time signature.
#[deriving(Show, Copy, Clone, RustcEncodable, RustcDecodable, PartialEq, Eq)]
pub struct TimeSig {
    pub top: Top,
    pub bottom: Bottom,
}

impl TimeSig {
    /// Return how many beats there are in a bar under this time signature.
    #[inline]
    pub fn beats_in_a_bar(&self) -> f64 {
        4.0 * self.top as f64 / self.bottom as f64
    }
}

