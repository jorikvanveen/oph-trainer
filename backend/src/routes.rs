use axum::Json;

use crate::dto::SampleDto;

pub async fn sample() -> Json<SampleDto> {
    Json(SampleDto {
        message: "Hello from Orpheus interview trainer".to_owned(),
    })
}
