pub fn red_bold(text: &str) -> String {
    format!("\x1b[31m\x1b[1m{}\x1b[m", text)
}
