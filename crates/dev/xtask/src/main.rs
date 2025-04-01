mod tools;

use {
    clap::{Parser, Subcommand},
    xshell::{Shell, cmd},
};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
enum SubCommands {
    /// Create `dist` directory containing release files
    Dist,

    /// Run checks for CI. Checks formatting, lints, and unit tests, in that order
    Check,

    /// Format code
    /// Only useful if using nightly fmt options or additional formatters for non-rust code
    Fmt,

    /// Install dev tools locally
    InstallDevTools,

    /// Clean
    Clean {
        #[arg(short, long)]
        dev_tools: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        SubCommands::Dist => {
            make_dist()
        }
        SubCommands::Check => {
            println!("CI Checks");
            checks().map_err(|_| anyhow::anyhow!("Checks failed! Please fix before committing"))
        }
        SubCommands::Fmt => {
            println!("Formatting code...");
            let sh = new_shell()?;
            // cmd!(sh, "cargo +nightly fmt").run()?;
            cmd!(sh, "cargo fmt").run()?;
            cmd!(sh, "taplo fmt").run()?;
            Ok(())
        }
        SubCommands::InstallDevTools => tools::install(),
        SubCommands::Clean { dev_tools } => {
            let sh = new_shell()?;
            if dev_tools {
                println!("Cleaning dev tools...");
                sh.remove_path(".deps")?;
            }
            println!("Cleaning miscellaneous files...");
            sh.remove_path("lcov.info")?;
            Ok(())
        }
    }
}

fn checks() -> anyhow::Result<()> {
    let sh = new_shell()?;
    println!("Checking format...");
    // cmd!(sh, "cargo +nightly fmt --check").run()?;
    cmd!(sh, "cargo fmt --check").run()?;
    cmd!(sh, "taplo fmt --check").run()?;
    println!("Checking lints...");
    cmd!(sh, "cargo clippy --quiet").run()?;
    println!("Checking unit tests...");
    cmd!(sh, "cargo llvm-cov").run()?;
    cmd!(sh, "cargo llvm-cov report --lcov --output-path lcov.info").run()?;
    Ok(())
}

fn workspace_dir() -> std::path::PathBuf {
    let mut p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p.pop();
    p
}

fn dev_tools_dir() -> std::path::PathBuf {
    let mut p = workspace_dir();
    p.push(".deps");
    p.push("bin");
    p
}

fn new_shell() -> anyhow::Result<Shell> {
    // Create shell
    let sh = Shell::new()?;

    // Prepend our dev dependencies to path
    let new_path = format!(
        "{}:{}",
        dev_tools_dir()
            .to_str()
            .ok_or(anyhow::anyhow!("Could not convert path to str"))?,
        sh.var("PATH")?
    );
    sh.set_var("PATH", new_path);

    Ok(sh)
}

fn make_dist() -> anyhow::Result<()> {
    let sh = new_shell()?;

    let targets = [
        "x86_64-unknown-linux-gnu",
    ];
    let bins = [
        "demo-bin",
    ];

    sh.remove_path("./dist")?;
    sh.create_dir("./dist")?;

    for target in targets {
        let dist_dir = sh.create_dir(format!("dist/{target}/"))?;
        cmd!(sh, "cargo build --release --target {target}").run()?;
        for bin in bins {
            sh.copy_file(format!("target/{target}/release/{bin}"), &dist_dir)?;
            cmd!(sh, "tar -czf dist/{target}.tgz dist/{target}/{bin}").run()?;
        }
        cmd!(sh, "tar -czf dist/{target}.tgz -C dist/{target} .").run()?;
    }


    Ok(())
}
