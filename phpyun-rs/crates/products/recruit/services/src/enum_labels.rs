//! Fixed enums: store codes, display via numbered keys + `label()` (request lang).
//! Do not hardcode Chinese in match arms; do not `if lang == en` concatenate copy.

use phpyun_core::i18n;

/// Lookup `messages.{key}` then `errors.{key}` then the bare key.
pub fn label(key: &str) -> String {
    let lang = i18n::current_lang();
    for prefix in ["messages.", "errors."] {
        let prefixed = format!("{prefix}{key}");
        let t = i18n::t(&prefixed, lang);
        if t != prefixed {
            return t;
        }
    }
    let t = i18n::t(key, lang);
    if t != key {
        t
    } else {
        key.to_string()
    }
}

fn key_or_empty(key: Option<&str>) -> String {
    key.map(label).unwrap_or_default()
}

pub fn pay_key(code: &str) -> Option<&'static str> {
    Some(match code {
        "alipay" => "wap_user_00319",
        "tenpay" => "common_07001",
        "bank" => "admin_system_00529",
        "alipaydual" => "common_07002",
        "alipayescow" => "common_07003",
        "adminpay" => "common_01630",
        "balance" => "common_07004",
        "admincut" => "common_07005",
        "wapalipay" => "common_06173",
        _ => return None,
    })
}

pub fn pay_name(code: &str) -> String {
    key_or_empty(pay_key(code))
}

pub fn order_kind_key(kind: i32) -> Option<&'static str> {
    Some(match kind {
        1 => "default_00090",
        2 => "admin_yunying_00095",
        3 => "admin_system_00529",
        4 => "wap_01041",
        5 => "wap_01233",
        10 => "wap_com_00238",
        11 => "member_com_00247",
        12 => "wap_com_00237",
        13 => "wap_com_00239",
        14 => "wap_user_00207",
        16 => "wap_com_00029",
        17 => "wap_01231",
        19 => "wap_00451",
        20 => "wap_00322",
        21 => "wap_00321",
        23 => "wap_com_00046",
        24 => "default_00030",
        25 => "wap_js_00130",
        28 => "wap_com_00039",
        _ => return None,
    })
}

pub fn order_kind_name(kind: i32) -> String {
    key_or_empty(order_kind_key(kind))
}

pub fn pay_state_key(state: i32) -> Option<&'static str> {
    Some(match state {
        0 => "admin_01264",
        1 => "admin_yunying_00085",
        2 => "admin_01265",
        3 => "admin_yunying_00086",
        4 => "admin_yunying_00078",
        _ => return None,
    })
}

pub fn pay_state_text(state: i32) -> String {
    key_or_empty(pay_state_key(state))
}

pub fn pay_state_html(state: i32) -> String {
    let text = pay_state_text(state);
    if text.is_empty() {
        return text;
    }
    let color = match state {
        0 | 4 => "red",
        1 => "green",
        2 => "#3d7dfd",
        3 => "#c30ad9",
        _ => return text,
    };
    format!("<font color={color}>{text}</font>")
}

pub fn sex_n(sex: i32) -> String {
    match sex {
        1 => label("common_02092"),
        2 => label("common_02069"),
        _ => String::new(),
    }
}

pub fn sex_or_unlimited(sex: i32) -> String {
    match sex {
        1 => label("common_02092"),
        2 => label("common_02069"),
        _ => label("common_01936"),
    }
}

pub fn sex_name(id: i32) -> String {
    sex_n(id)
}

/// search_list keys — Vue `localizeSearchList` / `lc()` (PHP numbered pack).
pub const SOURCE_LIST: &[(&str, &str)] = &[
    ("1", "member_user_00094"),
    ("2", "member_user_00163"),
    ("4", "wap_com_00249"),
    ("6", "resume_00043"),
    ("8", "model_00095"),
    ("9", "default_00250"),
    ("10", "common_07008"),
    ("11", "ajax_00010"),
    ("12", "wap_00121"),
    ("21", "wap_user_00339"),
    ("26", "common_07009"),
];

fn source_label_key(id: i32) -> Option<&'static str> {
    Some(match id {
        1 => "common_07024",
        2 => "member_user_00163",
        4 => "wap_com_00249",
        6 => "resume_00043",
        8 => "model_00095",
        9 => "default_00250",
        10 => "common_07008",
        11 => "common_07025",
        12 => "common_07026",
        21 => "wap_user_00339",
        26 => "common_07009",
        _ => return None,
    })
}

pub fn source_name(id: i32) -> String {
    key_or_empty(source_label_key(id))
}

pub fn wx_bind_msg(wxid: &str, unionid: &str) -> String {
    match (wxid.is_empty(), unionid.is_empty()) {
        (true, _) => label("common_02122"),
        (_, true) => label("common_02121"),
        _ => label("common_07010"),
    }
}

pub fn sms_result_n(state: i32) -> String {
    let key = match state {
        0 => return String::new(),
        401 => "common_07011",
        402 => "common_07012",
        403 => "common_07013",
        404 => "common_07014",
        405 => "common_07015",
        406 => "common_07016",
        407 => "common_07017",
        410 => "common_07018",
        411 => "common_07019",
        412 => "common_07020",
        413 => "common_07021",
        501 => "common_07022",
        502 => "common_07023",
        n => return n.to_string(),
    };
    label(key)
}

pub fn yes_no_key(v: &str) -> &'static str {
    match v {
        "1" => "common_02085",
        "2" => "common_02063",
        _ => "",
    }
}

pub fn sex_filter_key(v: &str) -> &'static str {
    match v {
        "1" => "common_02092",
        "2" => "common_02069",
        "3" => "common_01936",
        _ => "",
    }
}

pub fn unlimited() -> String {
    label("common_01936")
}

pub fn several() -> String {
    label("admin_user_company_00328")
}

/// Template-cache module id → numbered key (`set_config_settplcache`).
pub const TPL_CACHE_MODELS: &[(&str, &str)] = &[
    ("job", "default_00246"),
    ("resume", "default_00312"),
    ("part", "wap_user_00220"),
    ("company", "default_00114"),
    ("wap", "common_07006"),
    ("article", "default_00154"),
    ("announcement", "wap_00221"),
    ("hr", "default_00138"),
    ("zph", "member_com_00293"),
    ("ask", "wap_user_00223"),
    ("evaluate", "default_00126"),
    ("once", "wap_js_00130"),
    ("tiny", "wap_js_00066"),
    ("redeem", "wap_00398"),
    ("map", "wap_00317"),
    ("special", "wap_com_00310"),
    ("login", "wap_js_00145"),
    ("register", "wap_00242"),
    ("gongzhao", "default_00134"),
    ("error", "common_07007"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pay_and_order_labels_en_default() {
        assert_eq!(pay_name("adminpay"), "Manager Recharge");
        assert_eq!(order_kind_name(1), "Purchase member");
        assert_eq!(pay_state_text(2), "Payment successful");
        assert!(pay_state_html(2).contains("Payment successful"));
        assert!(!pay_state_html(2).contains("支付成功"));
        assert_eq!(sex_or_unlimited(1), "Male");
        assert_eq!(source_name(1), "Web");
    }
}
