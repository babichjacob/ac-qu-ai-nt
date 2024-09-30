use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(
        long,
        env,
        default_value_os_t = dirs_next::data_local_dir().expect("sorry but you're on a platform where dirs_next::data_local_dir() returned None, so please specify a data directory for the application").join("ac-qu-ai-nt")
    )]
    application_data_directory: PathBuf,
}

fn main() {
    let args = Args::parse();

    #[cfg(feature = "cli-clap")]
    ac_qu_ai_nt_cli_clap::main();
}
