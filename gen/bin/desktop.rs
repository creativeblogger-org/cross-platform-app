
fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    creative_blogger::start_app();
}
