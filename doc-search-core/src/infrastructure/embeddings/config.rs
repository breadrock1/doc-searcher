use gset::Getset;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize, Getset)]
pub struct EmbeddingsConfig {
    #[getset(get, vis = "pub")]
    address: String,
    /// Таймаут одного запроса (секунды).
    #[getset(get_copy, vis = "pub")]
    timeout: u64,
    /// Количество retry после первой попытки.
    #[getset(get_copy, vis = "pub")]
    retries: u32,
    /// Имя модели, передаваемое сервису.
    #[getset(get, vis = "pub")]
    model: String,
    /// Overlap (передаётся сервису в теле запроса).
    #[getset(get_copy, vis = "pub")]
    overlap: f64,
    /// Ожидаемая размерность вектора (валидация ответа).
    #[getset(get_copy, vis = "pub")]
    dimension: u32,
}
