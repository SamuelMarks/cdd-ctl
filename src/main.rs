fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", cdd::run()?);
    Ok(())
}
