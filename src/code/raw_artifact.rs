use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Language {
    Haskell,
    Rust,
    Python,
    Go,
    C,
    OCaml,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct MetaYaml {
    pub build: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct RawArticle {
    pub title: String,
    pub language: String,
    pub status: String,
    pub tags: Vec<String>,
    pub code: String,
    pub desc: String,
}

#[derive(Debug, Serialize)]
struct BuiltYaml {
    artifacts: Vec<RawArticle>,
    meta: MetaYaml,
}
