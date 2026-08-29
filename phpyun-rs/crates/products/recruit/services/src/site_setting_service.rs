//! Site settings (aligned with PHPYun `sy_*` global toggles).
//!
//! Public endpoint: read-only access to keys with `is_public=1`. Admin endpoint: full access plus create/update/delete.

use phpyun_core::{audit, clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::bank::repo as bank_repo;
use phpyun_models::site_setting::{entity::SiteSetting, repo as setting_repo};
use serde_json::{json, Map, Value};

pub async fn list_public(state: &AppState) -> AppResult<Vec<SiteSetting>> {
    Ok(setting_repo::list_public(state.db.reader()).await?)
}

pub async fn get(state: &AppState, key: &str) -> AppResult<Option<SiteSetting>> {
    Ok(setting_repo::find(state.db.reader(), key).await?)
}

// ---------- admin ----------

pub async fn admin_list(state: &AppState, user: &AuthenticatedUser) -> AppResult<Vec<SiteSetting>> {
    user.require_admin()?;
    Ok(setting_repo::list_all(state.db.reader()).await?)
}

pub struct UpsertInput<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub description: &'a str,
    pub is_public: bool,
}

pub async fn admin_upsert(
    state: &AppState,
    user: &AuthenticatedUser,
    input: UpsertInput<'_>,
) -> AppResult<()> {
    user.require_admin()?;
    let now = clock::now_ts();
    setting_repo::upsert(
        state.db.pool(),
        input.key,
        input.value,
        input.description,
        input.is_public,
        now,
    )
    .await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.site_setting.upsert", audit::Actor::uid(user.uid))
            .target(format!("key:{}", input.key)),
    )
    .await;
    Ok(())
}

pub async fn admin_delete(state: &AppState, user: &AuthenticatedUser, key: &str) -> AppResult<()> {
    user.require_admin()?;
    setting_repo::delete(state.db.pool(), key).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.site_setting.delete", audit::Actor::uid(user.uid))
            .target(format!("key:{key}")),
    )
    .await;
    Ok(())
}

fn cfg_map(rows: &[SiteSetting]) -> Map<String, Value> {
    let mut out = Map::new();
    for s in rows {
        out.insert(s.key_name.clone(), Value::String(s.value.clone()));
    }
    out
}

fn pick_map(cfg: &Map<String, Value>, keys: &[(&str, &str)]) -> Map<String, Value> {
    let mut out = Map::new();
    for (k, default) in keys {
        let v = cfg.get(*k).and_then(|x| x.as_str()).unwrap_or(*default);
        out.insert((*k).to_string(), Value::String(v.to_string()));
    }
    out
}

const ALIPAY_KEYS: &[(&str, &str)] = &[
    ("alipaytype", "1"),
    ("sy_alipayname", ""),
    ("sy_alipayKeyType", "1"),
    ("sy_alipayid", ""),
    ("sy_alipaycode", ""),
    ("sy_alipayemail", ""),
    ("sy_alipayappid", ""),
    ("sy_alipayprivatekey", ""),
    ("sy_alipaypublickey", ""),
    ("sy_weburl", ""),
];

const TENPAY_KEYS: &[(&str, &str)] = &[
    ("sy_tenpayid", ""),
    ("sy_tenpaycode", ""),
    ("sy_weburl", ""),
];

/// PHP `set_payset::index_action`: `{config, alipaydata, tenpaydata, bankrows}`.
pub async fn payset_index(state: &AppState, user: &AuthenticatedUser) -> AppResult<Value> {
    user.require_admin()?;
    let rows = setting_repo::list_all(state.db.reader()).await?;
    let config = cfg_map(&rows);
    let alipaydata = pick_map(&config, ALIPAY_KEYS);
    let tenpaydata = pick_map(&config, TENPAY_KEYS);
    let bankrows = bank_repo::list_all(state.db.reader()).await?;
    Ok(json!({
        "config": Value::Object(config),
        "alipaydata": Value::Object(alipaydata),
        "tenpaydata": Value::Object(tenpaydata),
        "bankrows": bankrows,
    }))
}

async fn upsert_keys(
    state: &AppState,
    user: &AuthenticatedUser,
    pairs: &[(&str, String)],
) -> AppResult<()> {
    for (k, v) in pairs {
        if k.is_empty() || *k == "pay_config" || *k == "config" {
            continue;
        }
        admin_upsert(
            state,
            user,
            UpsertInput {
                key: k,
                value: v,
                description: "",
                is_public: false,
            },
        )
        .await?;
    }
    Ok(())
}

fn str_field(v: &Value, key: &str) -> String {
    v.get(key)
        .map(|x| match x {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => {
                if *b {
                    "1".into()
                } else {
                    "0".into()
                }
            }
            Value::Null => String::new(),
            other => other.to_string(),
        })
        .unwrap_or_default()
}

fn or_weburl(body: &Value, weburl: &str) -> String {
    let s = str_field(body, "sy_weburl");
    if s.is_empty() {
        weburl.to_string()
    } else {
        s
    }
}

/// PHP `set_payset::alipay_action` — persist keys to `phpyun_admin_config`.
pub async fn payset_alipay(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<()> {
    user.require_admin()?;
    let weburl = setting_repo::find(state.db.reader(), "sy_weburl")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    upsert_keys(
        state,
        user,
        &[
            ("alipaytype", str_field(body, "alipaytype")),
            ("sy_alipayname", str_field(body, "sy_alipayname")),
            ("sy_alipayKeyType", str_field(body, "sy_alipayKeyType")),
            ("sy_alipayid", str_field(body, "sy_alipayid")),
            ("sy_alipaycode", str_field(body, "sy_alipaycode")),
            ("sy_alipayemail", str_field(body, "sy_alipayemail")),
            ("sy_alipayappid", str_field(body, "sy_alipayappid")),
            ("sy_alipayprivatekey", str_field(body, "sy_alipayprivatekey")),
            ("sy_alipaypublickey", str_field(body, "sy_alipaypublickey")),
            ("sy_weburl", or_weburl(body, &weburl)),
        ],
    )
    .await
}

/// PHP `set_payset::tenpay_action`.
pub async fn payset_tenpay(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &Value,
) -> AppResult<()> {
    user.require_admin()?;
    let weburl = setting_repo::find(state.db.reader(), "sy_weburl")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    upsert_keys(
        state,
        user,
        &[
            ("sy_tenpayid", str_field(body, "sy_tenpayid")),
            ("sy_tenpaycode", str_field(body, "sy_tenpaycode")),
            ("sy_weburl", or_weburl(body, &weburl)),
        ],
    )
    .await
}

pub struct BankIn<'a> {
    pub id: Option<u64>,
    pub name: &'a str,
    pub bank_name: &'a str,
    pub bank_number: &'a str,
    pub bank_address: &'a str,
}

/// PHP `set_payset::bank_action` — unique `bank_number`.
pub async fn payset_bank_upsert(
    state: &AppState,
    user: &AuthenticatedUser,
    input: BankIn<'_>,
) -> AppResult<u64> {
    user.require_admin()?;
    if let Some(existing) = bank_repo::find_by_number(state.db.reader(), input.bank_number).await? {
        let clash = match input.id.filter(|i| *i > 0) {
            Some(id) => existing.id != id,
            None => true,
        };
        if clash {
            return Err(ApiError::business("admin_system_00054"));
        }
    }
    let id = bank_repo::upsert(
        state.db.pool(),
        bank_repo::BankUpsert {
            id: input.id,
            name: input.name,
            bank_name: input.bank_name,
            bank_number: input.bank_number,
            bank_address: input.bank_address,
        },
    )
    .await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.payset.bank.upsert", audit::Actor::uid(user.uid))
            .target(format!("bank:{id}")),
    )
    .await;
    Ok(id)
}

/// PHP `set_payset::del_action`.
pub async fn payset_bank_delete(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    user.require_admin()?;
    bank_repo::delete(state.db.pool(), id).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.payset.bank.delete", audit::Actor::uid(user.uid))
            .target(format!("bank:{id}")),
    )
    .await;
    Ok(())
}
