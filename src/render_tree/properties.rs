/// Rectangular dimensions commonly used for certain properties such as margin/padding
#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub top: f64,
    pub left: f64,
    pub right: f64,
    pub bottom: f64,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            top: 0.,
            left: 0.,
            right: 0.,
            bottom: 0.,
        }
    }
}

impl Rectangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_values(top: f64, left: f64, right: f64, bottom: f64) -> Self {
        Self {
            top,
            left,
            right,
            bottom,
        }
    }
}
