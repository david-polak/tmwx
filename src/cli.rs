pub mod cli {
    use clap::Parser;

    #[derive(Parser)]
    struct Cli {
        pattern: String,
        #[clap(parse(from_os_str))]
        path: std::path::PathBuf,
    }

    pub fn initCli() {
        let args = Cli::parse();
    }
}
