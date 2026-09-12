pub mod entity;
pub mod repo;

/// Infer ATS kind from a careers URL and optional public API URL.
pub fn infer_provider(url: &str, api_url: &str) -> &'static str {
    let blob = format!("{} {}", url.trim(), api_url.trim()).to_ascii_lowercase();
    if blob.contains("greenhouse.io") || blob.contains("boards-api.greenhouse.io") {
        "greenhouse"
    } else if blob.contains("ashbyhq.com") {
        "ashby"
    } else if blob.contains("lever.co") {
        "lever"
    } else if blob.contains("myworkdayjobs.com") {
        "workday"
    } else {
        "other"
    }
}

pub fn normalize_provider(raw: &str, url: &str, api_url: &str) -> String {
    match raw.trim().to_ascii_lowercase().as_str() {
        "greenhouse" | "ashby" | "lever" | "workday" | "other" => {
            raw.trim().to_ascii_lowercase()
        }
        _ => infer_provider(url, api_url).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infer_known_hosts() {
        assert_eq!(
            infer_provider("https://job-boards.greenhouse.io/anthropic", ""),
            "greenhouse"
        );
        assert_eq!(
            infer_provider("https://jobs.ashbyhq.com/elevenlabs", ""),
            "ashby"
        );
        assert_eq!(infer_provider("https://jobs.lever.co/mistral", ""), "lever");
        assert_eq!(
            infer_provider(
                "https://nvidia.wd5.myworkdayjobs.com/NVIDIAExternalCareerSite",
                ""
            ),
            "workday"
        );
        assert_eq!(infer_provider("https://openai.com/careers", ""), "other");
    }
}
