use anyhow::Result;

fn main() {
    let output = read_file();
    println!("{:?}", output);
}

fn read_file() -> Result<()> {
    std::fs::read_to_string("file.txt")?;
    Ok(())
}