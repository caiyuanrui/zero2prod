#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}
