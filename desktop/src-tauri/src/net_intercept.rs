// Placeholder for request interception hooks
pub fn should_block(url: &str) -> bool {
    url.contains("/ads/") || url.contains("doubleclick")
}
