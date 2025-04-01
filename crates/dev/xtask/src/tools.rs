// This function is kept in its own file for caching purposes

use crate::new_shell;
use xshell::cmd;

pub fn install() -> anyhow::Result<()> {
    let sh = new_shell()?;

    println!("Creating .deps folder...");
    sh.create_dir(".deps")?;

    println!("Install cargo extensions...");
    let tools = [
        "cargo-llvm-cov@0.6.16", // Code coverage tool
        "taplo-cli@0.9.3",       // `Even Better Toml`'s backend for formatting TOML
    ];
    cmd!(sh, "cargo install --root .deps {tools...}").run()?;

    Ok(())
}
