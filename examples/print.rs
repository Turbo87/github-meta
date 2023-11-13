use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let meta = github_meta::meta()?;
    println!("{meta:#?}");
    let secret_scanning = github_meta::secret_scanning()?;
    println!("{secret_scanning:#?}");

    Ok(())
}
