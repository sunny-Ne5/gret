use anyhow::Context;

//Checks if pattern present in current line
pub fn check_match(line: &String, pattern: &String, mut writer: impl std::io::Write) {
    if line.contains(pattern) {
        write!(writer, "{}", line).with_context(|| format!("couldn't write data to output"));
    }
}
