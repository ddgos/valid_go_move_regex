use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use clap_stdin::FileOrStdin;
use goban::rules::game::Game;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Ordering {
    XY,
    YX,
}

impl ToString for Ordering {
    fn to_string(&self) -> String {
        match self {
            Ordering::XY => "xy",
            Ordering::YX => "yx",
        }
        .into()
    }
}

#[derive(Parser)]
#[command(version)]
struct Args {
    /// whitespace separated list of x labels to use
    x_labels: String,
    /// whitespace separated list of y labels to use
    y_labels: String,
    sgf: FileOrStdin,
    /// whether to display coordinates as xy or yx
    #[arg(default_value_t = Ordering::XY)]
    ordering: Ordering,
}

fn main() -> Result<()> {
    let Args {
        sgf,
        x_labels,
        y_labels,
        ordering,
    } = Args::parse();

    let x_labels: Vec<&str> = x_labels.split_whitespace().collect();
    let y_labels: Vec<&str> = y_labels.split_whitespace().collect();

    let game = Game::from_sgf(&sgf.contents()?).map_err(|e| anyhow!(e))?;

    let (width, height) = game.size();
    if width as usize > x_labels.len() {
        return Err(anyhow!(
            "Not enough x labels supplied. width: {}, x labels: {:?}",
            width,
            x_labels
        ));
    }
    if height as usize > y_labels.len() {
        return Err(anyhow!(
            "Not enough y labels supplied. height: {}, y labels: {:?}",
            height,
            y_labels
        ));
    }

    let legal_moves: Vec<_> = game
        .legals()
        .map(|(x, y)| {
            let x_label = x_labels
                .get(x as usize)
                .expect("should have checked there were enough labels");
            let y_label = y_labels
                .get(y as usize)
                .expect("should have checked there were enough labels");
            match ordering {
                Ordering::XY => (x_label, y_label),
                Ordering::YX => (y_label, x_label),
            }
        })
        .map(|(first, second)| format!("{}{}", first, second))
        .collect();
    print!("({})", legal_moves.join("|"));

    Ok(())
}
