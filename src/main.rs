use std::process::Command;

fn main() {
    println!("Hello, world!");
}

enum Choice {
    StartGame,
    AddGame,
    AddEmu,
    ReadGameDatabase,
    ReadEmuDatabase,
}

impl std::str::FromStr for Choice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choice_int: u8 = s.parse()?;
        match choice_int {
            1 => Ok(Self::StartGame),
            2 => Ok(Self::AddGame),
            3 => Ok(Self::AddEmu),
            4 => Ok(Self::ReadGameDatabase),
            5 => Ok(Self::ReadEmuDatabase),
            _ => Err(anyhow::anyhow!("Invalid choice")),
        }
    }
}

fn mainmenu() {
    println!(
        "Choose an action:
    (1) Start a game
    (2) Add a game
    (3) Add an emulator
    (4) Read the games database
    (5) Read the emulators database"
    );
}
