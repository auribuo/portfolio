#[macro_export]
macro_rules! rgb {
    ($r: expr, $g: expr, $b: expr) => {
        Color {
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl Color {
    pub(crate) fn hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub(crate) fn style_text(&self) -> String {
        format!("color: {}", self.hex())
    }
}
