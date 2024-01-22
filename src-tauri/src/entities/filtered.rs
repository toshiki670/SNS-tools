use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]

/// When the Filtered type is displayed with `std::fmt::Display`, it will show as `[FILTERED]`.
pub struct Filtered<T>(pub T);

impl<T> std::fmt::Display for Filtered<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[FILTERED]")
    }
}
