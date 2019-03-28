use cdd;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("{}", cdd::run()?);
    Ok(())
}
