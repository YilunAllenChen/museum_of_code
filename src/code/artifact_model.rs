use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Language {
    Haskell,
    Rust,
    Python,
    Go,
    C,
    OCaml,
    Bash,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct MetaYaml {
    pub build: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum ExhibitionHall {
    HallOfExpressiveness,
    HallOfTroll,
    HallOfSpeed,
    HallOfHelloWorld,
    Uncategorized,
}

impl Display for ExhibitionHall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let st = match self {
            ExhibitionHall::HallOfExpressiveness => "Hall of Expressiveness",
            ExhibitionHall::HallOfTroll => "Hall of Troll",
            ExhibitionHall::HallOfSpeed => "Hall of Speed",
            ExhibitionHall::HallOfHelloWorld => "Hall of Hello World",
            ExhibitionHall::Uncategorized => "Uncategorized",
        };
        write!(f, "{}", st)
    }
}

impl ExhibitionHall {
    pub fn desc(&self) -> &'static str {
        match self {
            ExhibitionHall::HallOfExpressiveness => {
                r#"The Hall of Expressiveness houses artifacts that are extremely elegant and concise,
                making them a joy to read and appreciate. They may not be the fastest, or the easiest 
                to read at first glance, but once you understand them, you will be amazed by the
                beauty of their design."#
            }
            ExhibitionHall::HallOfHelloWorld => {
                r#"The Hall of Hello World contains artifacts that bring back memories of the first time
                you wrote a program. Simple, yet powerful, these artifacts opens the door to the world
                of programming."#
            }
            ExhibitionHall::HallOfSpeed => {
                r#"Not all programs are created equal. The Hall of Speed contains artifacts that run 
                blazingly fast, achieving performance that is unmatched by others, making them the
                perfect choice for high-performance applications."#
            }
            ExhibitionHall::HallOfTroll => {
                r#"Who said programming has to be serious? The Hall of Troll contains artifacts that are
                designed to be funny, useless, or, even be dangerous! These artifacts are
                sure to make you laugh, or at least make you go "what the heck?""#
            }
            ExhibitionHall::Uncategorized => {
                r#"The Hall of Uncategorized contains artifacts that currently do not fit into any of the other
                halls."#
            }
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Article {
    pub title: String,
    pub language: Language,
    pub status: EntryStatus,
    pub tags: Vec<String>,
    pub code: String,
    pub desc: String,
    pub hall: Option<ExhibitionHall>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuiltYaml {
    pub artifacts: Vec<Article>,
    pub meta: MetaYaml,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug, Eq)]
pub enum EntryStatus {
    OnExhibit,
    StagedForExhibit,
    Maintenance,
}

impl EntryStatus {
    fn priority(&self) -> u8 {
        match self {
            EntryStatus::OnExhibit => 0,
            EntryStatus::StagedForExhibit => 100,
            EntryStatus::Maintenance => 200,
        }
    }
}

impl PartialOrd for EntryStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EntryStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}
