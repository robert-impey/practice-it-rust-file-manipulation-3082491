use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let file_path = "my_file";
    let contents = "This is my new file! Congrats!!!".to_string();

    // https://doc.rust-lang.org/stable/std/io/trait.Write.html#method.write_all
    let mut buffer = File::create(file_path)?;
    buffer.write_all(contents.as_bytes())?;
    Ok(())
}
