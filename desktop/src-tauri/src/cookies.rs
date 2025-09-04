use http::HeaderMap;

pub enum CookieDecision { AllowEssential, Block }

pub fn is_essential(_domain: &str, name: &str) -> CookieDecision {
    let essential = ["session", "csrf", "auth", "prefs"];
    if essential.iter().any(|k| name.to_lowercase().contains(k)) {
        CookieDecision::AllowEssential
    } else {
        CookieDecision::Block
    }
}

pub fn filter_set_cookie(headers: &mut HeaderMap) {
    if let Some(val) = headers.get(http::header::SET_COOKIE) {
        let s = val.to_str().unwrap_or("");
        let parts: Vec<&str> = s.split(';').collect();
        if let Some((name, _)) = parts[0].split_once('=') {
            if let CookieDecision::Block = is_essential("", name) {
                headers.remove(http::header::SET_COOKIE);
            }
        }
    }
}
