pub fn check_match(line: &String, pattern: &str, mut writer: impl std::io::Write) {
    if line.contains(pattern) {
        writeln!(writer, "{}", line); //write bytes into writer
    }
}