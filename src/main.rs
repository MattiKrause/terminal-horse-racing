use std::io::stdin;
use std::process::exit;
use std::time::{Instant, SystemTime};
use term::color;

static HORSE_COLORS: [color::Color; 5] = [color::RED, color::GREEN, color::MAGENTA, color::BLUE, color::CYAN];
#[derive(Clone)]
struct Horse { advance: u32 }

fn main() {
    let Some(mut terminal) = term::stdout() else {
        eprintln!("console not supported");
        exit(-1);
    };
    run_game(terminal.as_mut()).unwrap();
}

const WINNING_NUM: u32 = 50;
const MAX_ADVANCE: u128 = 4;

fn run_game(terminal: &mut term::StdoutTerminal) -> std::io::Result<()> {
    use std::io::Read;
    let mut horses = vec![Horse { advance: 0 }; 7];
    write_horses(terminal, &horses).unwrap();
    let mut stdin = stdin();
    let mut line = String::new();
    while horses.iter().all(|h| h.advance < WINNING_NUM) {
        let _ = stdin.read_line(&mut line);

        let time_since_unix = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        tick_horses(&mut horses, time_since_unix.as_millis(), MAX_ADVANCE);

        terminal.cursor_up()?;
        delete_horses(terminal, horses.len())?;
        //terminal.carriage_return()?;
        write_horses(terminal, &horses)?;
    }
    write_winning_state(terminal, &horses)
}

fn tick_horses(horses: &mut [Horse], tick_by: u128, max_advance: u128) {
    let _ = horses.iter_mut().fold(tick_by, |acc, horse| {
        horse.advance += ((acc % max_advance) + 1) as u32;
        acc / max_advance
    });
}

fn write_winning_state(terminal: &mut term::StdoutTerminal, horses: &[Horse]) -> std::io::Result<()> {
    let mut horses_by_advanced = to_horse_iter(&horses).collect::<Vec<_>>();
    horses_by_advanced.sort_by(|(_, h1,_), (_, h2, _)| h1.advance.cmp(&h2.advance).reverse());
    let (winner_horse, _, winning_horse_color) = *horses_by_advanced.first().unwrap();
    delete_horses(terminal, horses_by_advanced.len())?;
    write_horses_iter(terminal, horses_by_advanced.into_iter())?;
    let _ = terminal.fg(winning_horse_color);
    writeln!(terminal, "horse {winner_horse} won!")
}

fn delete_horses(terminal: &mut term::StdoutTerminal, horse_count: usize)-> std::io::Result<()>{
    terminal.carriage_return()?;
    for _ in 0..horse_count {
        terminal.cursor_up()?;
        terminal.delete_line()?;
    }
    Ok(())
}

fn write_horses_iter<'a>(term: &mut term::StdoutTerminal,mut horses: impl Iterator<Item = (usize, &'a Horse, color::Color)>) -> std::io::Result<()> {
    horses.try_for_each(|(horse_number, advance, c)| {
        let _ = term.fg(c);
        writeln!(term, "{}{}", "_".repeat(advance.advance as usize), horse_number)
    })
}

fn to_horse_iter(horses: &[Horse]) -> impl Iterator<Item=(usize, &'_ Horse, color::Color)> {
    horses.iter().enumerate().zip(HORSE_COLORS.iter().cycle().copied()).map(|((num, horse), color)| (num, horse, color))
}

fn write_horses(term: &mut term::StdoutTerminal, horses: &[Horse]) -> std::io::Result<()> {
    write_horses_iter(term, to_horse_iter(horses))
}
