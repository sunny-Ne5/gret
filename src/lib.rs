use anyhow::Context;
pub fn check_match(line: &String, pattern: &str, mut writer: impl std::io::Write) {
    if line.contains(pattern) {
        writeln!(writer, "{}", line).with_context(|| format!("couldn't write data to output"));
    }
}
