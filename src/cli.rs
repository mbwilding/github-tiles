use clap::Parser;

#[derive(Parser)]
#[command(about = "Generate GitHub stats SVGs")]
pub struct Args {
    /// GitHub token (https://github.com/settings/tokens/new?scopes=repo,read:user&description=GitHub%20Tiles)
    #[arg(long, env = "GITHUB_TOKEN")]
    pub token: String,

    /// Output directory
    #[arg(long, default_value = "assets")]
    pub output: String,

    /// Tiles to generate (comma-separated: statistics,languages,contributions)
    #[arg(long, default_value = "statistics,languages,contributions")]
    pub tiles: String,

    /// Include private repositories
    #[arg(long, default_value = "false")]
    pub private: bool,

    /// Include forked repositories
    #[arg(long, default_value = "false")]
    pub forks: bool,

    /// Languages to display
    #[arg(long, default_value = "5")]
    pub languages_limit: u8,

    /// Languages to ignore (comma-separated and case-insensitive)
    #[arg(long)]
    pub languages_exclude: Option<String>,

    /// Contributions to display
    #[arg(long, default_value = "10")]
    pub contributions_limit: u8,

    /// Contributions minimum stars
    #[arg(long, default_value = "0")]
    pub contributions_min_stars: u32,

    /// Render opaque background
    #[arg(long, default_value = "false")]
    pub opaque: bool,

    /// SVG optimization
    #[arg(long, default_value = "true")]
    pub optimize: bool,
}

impl Args {
    pub fn get() -> Self {
        Args::parse()
    }
}
