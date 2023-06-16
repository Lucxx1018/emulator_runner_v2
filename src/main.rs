use std::{process::Command, str::FromStr};

fn main() {
    println!("Hello, world!");
    let choice = mainmenu();
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
            _ => Err(anyhow::anyhow!("Invalid variant")),
        }
    }
}

fn input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn mainmenu() -> Choice {
    loop {
        println!(
            "Choose an action:
    (1) Start a game
    (2) Add a game
    (3) Add an emulator
    (4) Read the games database
    (5) Read the emulators database"
        );
        let choice_str = input();
        let choice = Choice::from_str(&choice_str);
        match choice {
            Ok(c) => break c,
            Err(_) => panic!("Fuck"),
        }
    }
}
