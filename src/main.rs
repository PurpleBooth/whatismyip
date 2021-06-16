use anyhow::anyhow;
use anyhow::Result;
use trust_dns_resolver::Resolver;

mod cli;

fn main() -> Result<()> {
    cli::app().get_matches();

    let error = anyhow!("No ip found");

    let resolver = Resolver::default()?;
    println!(
        "{}",
        resolver
            .txt_lookup("o-o.myaddr.l.google.com")?
            .iter()
            .next()
            .ok_or(error)?
    );

    Ok(())
}
