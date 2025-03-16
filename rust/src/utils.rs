pub fn get_home_dir() -> std::path::PathBuf {
    std::env::var("HOME").map(|h| std::path::PathBuf::from(h)).unwrap_or_else(|_| std::path::PathBuf::from("."))
}