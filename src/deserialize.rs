use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Color {
    pub color: String,
    pub value: String,
}
