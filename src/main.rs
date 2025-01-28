use anyhow::{anyhow, Result};
use clap::Parser;
use clap_stdin::FileOrStdin;
use goban::rules::game::Game;

#[derive(Parser)]
struct Args {
    sgf: FileOrStdin
}

fn coord_to_str(x: u8, y: u8) -> String {
    format!("{}{}", (b'a' + x) as char, y + 1)
}

fn main() -> Result<()> {
    let Args { sgf } = Args::parse();

    let game = Game::from_sgf(&sgf.contents()?).map_err(|e| anyhow!(e))?; 

    let (width, height) = game.size();
    if width > 19 {
        return Err(anyhow!("Maximum allowed board size 19. width: {}", width));
    }
    if height > 19 {
        return Err(anyhow!("Maximum allowed board size 19. height: {}", height));   
    }

    let legal_moves: Vec<_> = game.legals().map(|(x, y)| coord_to_str(x, y)).collect();
    print!("({})", legal_moves.join("|"));

    Ok(())
}
