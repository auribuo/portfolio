pub(crate) mod color;

use color::Color;

use crate::rgb;

#[derive(Debug, Clone, Copy)]
pub(crate) struct TerminalTheme {
    pub(crate) crust: Color,
    pub(crate) red: Color,
    pub(crate) green: Color,
    pub(crate) sapphire: Color,
    pub(crate) peach: Color,
    pub(crate) text: Color,
}

pub(crate) const MOCHA: TerminalTheme = TerminalTheme {
    red: rgb!(243, 139, 168),
    green: rgb!(166, 227, 161),
    sapphire: rgb!(116, 199, 236),
    peach: rgb!(250, 179, 135),
    text: rgb!(205, 214, 244),
    crust: rgb!(17, 17, 27),
};
