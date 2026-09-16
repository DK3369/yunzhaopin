//! Interview multi-dimension review. Distinct from star `ratings`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::date_parse::de_loose_u64;
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::interview_review_service::{self, Dim, ReviewView};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/interviews/review", post(get_mine))
        .route("/interviews/review/submit", post(submit))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReviewQuery {
    #[serde(default, deserialize_with = "de_loose_u64")]
    pub yqms_id: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate, ToSchema)]
pub struct DimIn {
    #[validate(length(min = 1, max = 32))]
    pub key: String,
    #[validate(range(min = 1, max = 5))]
    pub score: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReviewSubmitForm {
    #[serde(default, deserialize_with = "de_loose_u64")]
    pub yqms_id: u64,
    #[validate(length(min = 1, max = 10))]
    pub dimensions: Vec<DimIn>,
    #[serde(default)]
    #[validate(length(max = 1000))]
    pub comment: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReviewOut {
    pub submitted: bool,
    pub yqms_id: u64,
    pub dimensions: Vec<DimIn>,
    pub total: u32,
    pub comment: String,
}

impl From<ReviewView> for ReviewOut {
    fn from(v: ReviewView) -> Self {
        Self {
            submitted: v.submitted,
            yqms_id: v.yqms_id,
            dimensions: v
                .dimensions
                .into_iter()
                .map(|d| DimIn {
                    key: d.key,
                    score: d.score,
                })
                .collect(),
            total: v.total,
            comment: v.comment,
        }
    }
}

/// Read my review for a `userid_msg` invite. Empty when not submitted.
#[utoipa::path(
    post,
    path = "/v1/mcenter/interviews/review",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ReviewQuery,
    responses((status = 200, description = "ok", body = ReviewOut))
)]
pub async fn get_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<ReviewQuery>,
) -> AppResult<ApiResponse<ReviewOut>> {
    let v = interview_review_service::get_mine(&state, &user, q.yqms_id).await?;
    Ok(ApiResponse::data(ReviewOut::from(v)))
}

/// Write dimensions (`professional` / `communication` / `punctuality` or extra keys).
#[utoipa::path(
    post,
    path = "/v1/mcenter/interviews/review/submit",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ReviewSubmitForm,
    responses((status = 200, description = "ok", body = ReviewOut))
)]
pub async fn submit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ReviewSubmitForm>,
) -> AppResult<ApiResponse<ReviewOut>> {
    let dims: Vec<Dim> = f
        .dimensions
        .iter()
        .map(|d| Dim {
            key: d.key.clone(),
            score: d.score,
        })
        .collect();
    let v = interview_review_service::submit(&state, &user, f.yqms_id, &dims, &f.comment).await?;
    Ok(ApiResponse::data(ReviewOut::from(v)))
}
