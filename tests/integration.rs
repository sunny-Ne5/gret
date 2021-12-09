use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_doesn_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gret")?;
    cmd.arg("pattern").arg("/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not open file"));
    Ok(())
}

#[test]

fn file_exist_match_found() -> Result<(), Box<dyn std::error::Error>> {
    let path = env!("CARGO_MANIFEST_DIR"); //Gives stable path to gret's root directory
    let mut cmd = Command::cargo_bin("gret")?;
    cmd.arg("sunny").current_dir(&path).arg("Cargo.toml");
    cmd.assert().success().stdout(predicate::str::contains(
        "authors = [\"sunny-Ne5 <sunny.tiwari505@gmail.com>\"]",
    ));
    Ok(())
}
