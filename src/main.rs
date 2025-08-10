#[derive(Debug, Default)]
struct Args {
    dry_run: bool,
    check: bool,
}

impl Args {
    fn with_arg(mut self, arg: String) -> Self {
        match arg.as_str() {
            "--dry-run" => {
                self.dry_run = true;
            }
            "--check" => {
                self.check = true;
            }
            _ => {}
        };
        self
    }

    fn from_args() -> Self {
        std::env::args().fold(Self::default(), |acc, arg| acc.with_arg(arg))
    }

    #[inline]
    fn dry_run(&self) -> bool {
        self.dry_run || self.check
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    let crate_root = std::env::current_dir()?;

    let mut crates = cargo_hilly::CratesScanner::default();
    crates.scan(crate_root.clone());
    let crates = crates.into_inner();

    if crates.is_empty() {
        eprintln!("⚠️ no crates found in the current directory");
        std::process::exit(1);
    }

    let mut candidates = cargo_hilly::FlatModuleScanner::default();
    candidates.scan_crates(&crates);
    let candidates = candidates.into_inner();

    if candidates.is_empty() {
        println!("✅ No modules to migrate.");
        return Ok(());
    }

    let dry_run = args.dry_run();
    let migrator = cargo_hilly::ModuleMigrator;
    for (src, dst) in candidates.iter() {
        println!("🚀 {src:?} → {dst:?}");
        if !dry_run {
            migrator.migrate(src, dst)?;
        }
    }

    if args.check {
        eprintln!("⚠️ {} module(s) need to be migrated", candidates.len());
        std::process::exit(1);
    }

    println!("✅ Migrated {} module(s).", candidates.len());
    Ok(())
}
