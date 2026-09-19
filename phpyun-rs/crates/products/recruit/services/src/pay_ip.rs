//! HMAC merchant IP allowlist. Empty list is deny (fail-closed).

use std::net::{IpAddr, Ipv4Addr};

const MAX_CHARS: usize = 2048;
const MAX_TOKENS: usize = 64;

pub fn parse_allow_ips(raw: &str) -> Result<String, &'static str> {
    if raw.contains('\0') || raw.len() > MAX_CHARS {
        return Err("allow_ips");
    }
    let mut out: Vec<String> = Vec::new();
    for part in raw.split(|c: char| c == '\n' || c == '\r' || c == ',' || c == ';' || c == '|') {
        let tok = part.trim();
        if tok.is_empty() {
            continue;
        }
        if !token_ok(tok) {
            return Err("allow_ips");
        }
        if !out.iter().any(|x| x == tok) {
            out.push(tok.to_string());
        }
        if out.len() > MAX_TOKENS {
            return Err("allow_ips");
        }
    }
    Ok(out.join("\n"))
}

pub fn ip_allowed(allow_ips: &str, client_ip: &str) -> bool {
    let Some(client) = canon_ip(client_ip) else {
        return false;
    };
    for part in allow_ips.split(|c: char| c == '\n' || c == '\r' || c == ',' || c == ';' || c == '|')
    {
        let tok = part.trim();
        if tok.is_empty() {
            continue;
        }
        if !token_ok(tok) {
            continue;
        }
        if token_matches(tok, client) {
            return true;
        }
    }
    false
}

fn canon_ip(s: &str) -> Option<IpAddr> {
    let t = s.trim().trim_matches(|c| c == '[' || c == ']');
    let ip: IpAddr = t.parse().ok()?;
    match ip {
        IpAddr::V6(v6) => Some(v6.to_ipv4_mapped().map(IpAddr::V4).unwrap_or(ip)),
        v => Some(v),
    }
}

fn token_ok(tok: &str) -> bool {
    if tok.parse::<IpAddr>().is_ok() {
        return true;
    }
    ipv4_cidr(tok).is_some()
}

fn ipv4_cidr(tok: &str) -> Option<(Ipv4Addr, u8)> {
    let (net, bits) = tok.split_once('/')?;
    let prefix: u8 = bits.parse().ok()?;
    if prefix == 0 || prefix > 32 {
        return None;
    }
    let net: Ipv4Addr = net.parse().ok()?;
    Some((net, prefix))
}

fn token_matches(tok: &str, client: IpAddr) -> bool {
    if let Ok(listed) = tok.parse::<IpAddr>() {
        let listed = match listed {
            IpAddr::V6(v6) => v6.to_ipv4_mapped().map(IpAddr::V4).unwrap_or(listed),
            v => v,
        };
        return listed == client;
    }
    let Some((net, prefix)) = ipv4_cidr(tok) else {
        return false;
    };
    let IpAddr::V4(cli) = client else {
        return false;
    };
    let shift = 32 - prefix;
    let mask = if prefix == 32 {
        u32::MAX
    } else {
        u32::MAX << shift
    };
    (u32::from(net) & mask) == (u32::from(cli) & mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_deny() {
        assert!(!ip_allowed("", "1.2.3.4"));
        assert!(!ip_allowed("\n  \n", "1.2.3.4"));
        assert_eq!(parse_allow_ips("").unwrap(), "");
    }

    #[test]
    fn exact_v4_and_cidr() {
        let list = parse_allow_ips("127.0.0.1\n10.0.0.0/8").unwrap();
        assert!(ip_allowed(&list, "127.0.0.1"));
        assert!(ip_allowed(&list, "10.9.8.7"));
        assert!(!ip_allowed(&list, "11.0.0.1"));
        assert!(!ip_allowed(&list, "0.0.0.0"));
    }

    #[test]
    fn exact_v6() {
        let list = parse_allow_ips("::1").unwrap();
        assert!(ip_allowed(&list, "::1"));
        assert!(!ip_allowed(&list, "127.0.0.1"));
    }

    #[test]
    fn reject_slash_zero_and_garbage() {
        assert!(parse_allow_ips("0.0.0.0/0").is_err());
        assert!(parse_allow_ips("not-an-ip").is_err());
        assert!(!ip_allowed("10.0.0.0/8", "not-ip"));
    }
}
