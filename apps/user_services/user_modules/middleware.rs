use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::Json,
    Error, HttpMessage, HttpResponse,
};
use serde_json::json;
use validator::Validate;

use crate::common::utils::error::extract_errors;

pub async fn validate_json<T>(
    mut req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error>
where
    T: serde::de::DeserializeOwned + Validate + std::fmt::Debug + 'static,
{
    let payload = match req.extract::<Json<T>>().await {
        Ok(data) => data,
        Err(e) => {
            let res = HttpResponse::BadRequest().json(json!({
                "error": "invalid_json",
                "message": e.to_string()
            }));
            return Ok(req.into_response(res));
        }
    };

    if let Err(e) = payload.validate() {
        let res = HttpResponse::BadRequest().json(json!({
            "error": "validation_failed",
            "messages": extract_errors(e)
        }));
        return Ok(req.into_response(res));
    }

    req.extensions_mut().insert(payload.into_inner());
    next.call(req).await
}
