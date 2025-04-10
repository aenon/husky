fn main() {
    println!("Hello World!");
use static_site_gen::{Generator, StaticSiteGenerator};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let generator = StaticSiteGenerator::new("config.yml")?;
    generator.run().await?;
    Ok(())
}