#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn mix(&self, other: &Color) -> Color {
        let red: u16 = (self.red as u16 + other.red as u16) / 2;
        let green: u16 = (self.green as u16 + other.green as u16) / 2;
        let blue: u16 = (self.blue as u16 + other.blue as u16) / 2;
        Color {
            red: red as u8,
            green: green as u8,
            blue: blue as u8,
        }
    }
}
