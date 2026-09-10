//! Ad slot public read (matching PHPYun `ad.model.php`).

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use phpyun_core::dto::IdBody;
use phpyun_core::{
    ApiError, ApiResponse, AppResult, AppState, ClientIp, MaybeUser, ValidatedJson,
    ValidatedJsonOrQuery,
};
use phpyun_models::ad::entity::Ad;
use phpyun_services::ad_service;
use serde::de::{self, Deserializer, SeqAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &["/v1/wap/ads", "/v1/wap/initads"];

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ads", get(list).post(list))
        .route("/initads", get(initads).post(initads))
        .route("/ads/click", post(track_click))
}

#[derive(Debug, Deserialize, Serialize, Validate, IntoParams, ToSchema)]
pub struct AdQuery {
    /// Slot key — alphanumeric / underscore / hyphen, 1..=64 chars. The
    /// string flows into `phpyun_ad.slot = ?`, so a stricter charset guard
    /// (vs. plain length) removes any chance of control characters / spaces
    /// reaching the DB and keeps slot keys URL-safe.
    #[validate(length(min = 1, max = 64), custom(function = "validate_slot_charset"))]
    pub slot: String,
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 100))]
    pub limit: u64,
}
fn default_limit() -> u64 {
    10
}

fn validate_slot_charset(s: &str) -> Result<(), validator::ValidationError> {
    if s.bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        Ok(())
    } else {
        Err(validator::ValidationError::new("slot_charset"))
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdView {
    pub id: u64,
    pub title: String,
    pub image: String,
    /// Normalized full URL of `image` (with site / CDN prefix)
    pub image_n: String,
    pub link: String,
    /// 1=current window / 2=new window
    pub target: i32,
    pub pic_width: String,
    pub pic_height: String,
    pub pic_content: String,
}

fn to_view(state: &AppState, site_base: Option<&str>, a: Ad) -> AdView {
    AdView {
        image_n: state.storage.normalize_legacy_url(&a.image, site_base),
        id: a.id,
        title: a.title,
        image: a.image,
        link: a.link,
        target: a.target,
        pic_width: a.pic_width,
        pic_height: a.pic_height,
        pic_content: a.pic_content,
    }
}

/// List active ads for a slot
#[utoipa::path(post, path = "/v1/wap/ads", tag = "wap", params(AdQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<AdQuery>,
) -> AppResult<ApiResponse<Vec<AdView>>> {
    let list = ad_service::list_active(&state, &q.slot, q.limit).await?;
    let site_base = state.config.web_base_url.as_deref();
    let items = list
        .into_iter()
        .map(|a| to_view(&state, site_base, a))
        .collect();
    Ok(ApiResponse::data(items))
}

/// GET compact: `slots=3:5,50:5`. POST JSON: `{ "slots": [{ "slot": "3", "limit": 5 }] }`.
#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema)]
pub struct InitAdsInput {
    #[serde(default, deserialize_with = "de_slots")]
    #[validate(length(min = 1, max = 32), nested)]
    pub slots: Vec<AdQuery>,
}

fn parse_compact_slots(s: &str) -> Result<Vec<AdQuery>, String> {
    let mut out = Vec::new();
    for part in s.split([',', '|']) {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let (slot, limit) = match part.split_once(':') {
            Some((a, b)) => {
                let lim = b.trim().parse::<u64>().unwrap_or(10).clamp(1, 100);
                (a.trim().to_string(), lim)
            }
            None => (part.to_string(), 10),
        };
        if slot.is_empty() {
            continue;
        }
        out.push(AdQuery { slot, limit });
    }
    Ok(out)
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SlotElem {
    One(AdQuery),
    Spec(String),
}

fn de_slots<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<AdQuery>, D::Error> {
    struct SlotsVisitor;
    impl<'de> Visitor<'de> for SlotsVisitor {
        type Value = Vec<AdQuery>;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("slot list or compact spec like 3:5,50:5")
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            parse_compact_slots(v).map_err(E::custom)
        }
        fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
            self.visit_str(&v)
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut out = Vec::new();
            while let Some(item) = seq.next_element::<SlotElem>()? {
                match item {
                    SlotElem::One(q) => out.push(q),
                    SlotElem::Spec(s) => {
                        out.extend(parse_compact_slots(&s).map_err(de::Error::custom)?);
                    }
                }
            }
            Ok(out)
        }
        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(Vec::new())
        }
        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(Vec::new())
        }
    }
    deserializer.deserialize_any(SlotsVisitor)
}

/// Batch active ads. Keys are slot ids; empty slots return `[]`.
#[utoipa::path(
    post,
    path = "/v1/wap/initads",
    tag = "wap",
    request_body = InitAdsInput,
    responses((status = 200, description = "ok"))
)]
pub async fn initads(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<InitAdsInput>,
) -> AppResult<ApiResponse<BTreeMap<String, Vec<AdView>>>> {
    let needs: Vec<ad_service::SlotNeed> = q
        .slots
        .into_iter()
        .map(|s| ad_service::SlotNeed {
            slot: s.slot,
            limit: s.limit,
        })
        .collect();
    let map = ad_service::list_active_many(&state, &needs).await?;
    let site_base = state.config.web_base_url.as_deref();
    let out = map
        .into_iter()
        .map(|(k, list)| {
            (
                k,
                list.into_iter()
                    .map(|a| to_view(&state, site_base, a))
                    .collect(),
            )
        })
        .collect();
    Ok(ApiResponse::data(out))
}

// ==================== Click tracking ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct AdClickResp {
    /// Target URL the front-end should redirect to (matches PHP
    /// `phpyun_ad.pic_src`). Empty string when the ad row is missing.
    pub target_url: String,
    /// Whether this click was actually persisted (`false` when the per-IP
    /// rate limit was exceeded — PHP redirects without inserting in that case).
    pub recorded: bool,
}

/// Record an ad click. Counterpart of PHP `index/index::clickhits_action`.
/// Per-IP rate limit window is read from `sy_adclick` (hours); a 0 / unset
/// value disables the rate limit.
#[utoipa::path(post,
    path = "/v1/wap/ads/click",
    tag = "wap",
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = AdClickResp),
        (status = 404, description = "Ad not found"),
    )
)]
pub async fn track_click(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<AdClickResp>> {
    let id = b.id;
    use phpyun_models::ad::repo as ad_repo;
    let target = ad_repo::find_target(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("ad_not_found"))?;

    let now = phpyun_core::clock::now_ts();

    // Rate limit window in hours. PHP reads `sy_adclick` from
    // `phpyun_admin_config`; when the column is missing or 0 we skip the
    // rate-limit step (any non-zero value is treated as "hours").
    let window_hours = phpyun_models::site_setting::repo::find(state.db.reader(), "sy_adclick")
        .await
        .ok()
        .flatten()
        .and_then(|s| s.value.parse::<i64>().ok())
        .unwrap_or(0);
    let recorded = if window_hours > 0 {
        let since = now - window_hours * 3600;
        let n = ad_repo::count_clicks_recent(state.db.reader(), id, &ip, since).await?;
        if n > 0 {
            false
        } else {
            ad_repo::insert_click(
                state.db.pool(),
                id,
                user.as_ref().map(|u| u.uid).unwrap_or(0),
                &ip,
                now,
            )
            .await?;
            true
        }
    } else {
        ad_repo::insert_click(
            state.db.pool(),
            id,
            user.as_ref().map(|u| u.uid).unwrap_or(0),
            &ip,
            now,
        )
        .await?;
        true
    };

    Ok(ApiResponse::data(AdClickResp {
        target_url: target,
        recorded,
    }))
}
