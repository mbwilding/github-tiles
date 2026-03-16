mod cli;
mod github;
mod helpers;
mod icons;
mod svg;
mod theme;
mod tiles;

use crate::cli::Args;
use anyhow::{Context, Result};
use log::{debug, info};
use std::path::Path;
use tiles::{Contributions, Languages, RenderConfig, Statistics, Tile};
use tokio::fs;

use crate::helpers::comma_str_to_vec_slice;

#[tokio::main]
async fn main() -> Result<()> {
    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let filter = match rust_log.as_str() {
        "" => "warn,github_tiles=info".to_string(),
        "trace" => rust_log.to_string(),
        "debug" => format!("warn,github_tiles={}", rust_log),
        "info" | "warn" | "error" => format!("warn,github_tiles={}", rust_log),
        _ => {
            if rust_log.contains("github_tiles") {
                format!("warn,{}", rust_log)
            } else {
                format!("warn,github_tiles=info,{}", rust_log)
            }
        }
    };

    env_logger::Builder::new()
        .parse_filters(&filter)
        .format_target(false)
        .init();

    let args = Args::get();
    let client = reqwest::Client::new();

    info!("Fetching GitHub data...");

    let user_data = github::fetch_user_data(&client, &args.token, args.private).await?;
    let user = user_data
        .viewer
        .context("Failed to get authenticated user data")?;

    let username = &user.login;
    debug!("Authenticated as: {}", username);

    let tile_selection = comma_str_to_vec_slice(&args.tiles);

    let statistics = if tile_selection.contains(&"statistics") {
        Some(Statistics::from_user(&user, args.forks))
    } else {
        None
    };

    let excluded_languages = comma_str_to_vec_slice(&args.languages_exclude);
    let languages = if tile_selection.contains(&"languages") {
        Some(Languages::from_user(
            &user,
            args.languages_limit,
            args.forks,
            &excluded_languages,
        ))
    } else {
        None
    };

    let mut contributions = if tile_selection.contains(&"contributions") {
        Some(Contributions::from_user(
            &user,
            username,
            args.contributions_limit,
            args.contributions_min_stars,
        ))
    } else {
        None
    };

    if let Some(ref mut contrib) = contributions {
        info!("Fetching avatars...");
        contrib.fetch_avatars(&client).await?;
    }

    let output_path = Path::new(&args.output);
    debug!("Output directory: {}", output_path.display());
    fs::create_dir_all(output_path).await?;

    let mut tiles: Vec<&dyn Tile> = Vec::new();
    if let Some(ref s) = statistics {
        tiles.push(s);
    }
    if let Some(ref l) = languages {
        tiles.push(l);
    }
    if let Some(ref c) = contributions {
        tiles.push(c);
    }

    for theme in theme::ALL {
        let config = RenderConfig::new(theme, args.opaque);

        for tile in &tiles {
            let svg_content = tile.render(&config);
            let svg = if args.optimize {
                svg::optimize(&svg_content)
            } else {
                svg_content
            };
            let filename = tile.filename(theme.name);
            debug!("Writing: {}", filename);
            fs::write(output_path.join(&filename), &svg).await?;
        }

        info!("Generated {} theme SVGs", theme.name);
    }

    info!("Done! SVGs saved to {}/", args.output);

    Ok(())
}
