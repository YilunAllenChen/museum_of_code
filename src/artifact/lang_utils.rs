use std::fmt::Display;

use super::Language;
use crate::html_utils::render_text_tag;

impl PartialOrd for Language {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Language {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        format!("{:?}", self).cmp(&format!("{:?}", other))
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lan_str = match self {
            Language::Haskell => "Haskell",
            Language::Rust => "Rust",
            Language::Python => "Python",
            Language::Go => "Go",
            Language::C => "C",
            Language::OCaml => "OCaml",
            Language::Bash => "Bash",
            Language::Clojure => "Clojure",
        };
        write!(f, "{}", lan_str)
    }
}

impl Language {
    pub fn icon(&self) -> &'static str {
        // https://devicon.dev/
        match self {
            Language::Haskell => {
                r#"<i class="devicon-haskell-plain colored" style="font-size: 3rem"></i>"#
            }
            Language::Rust => r#"<i class="devicon-rust-plain" style="font-size: 3rem"></i>"#,
            Language::Python => {
                r#"<i class="devicon-python-plain colored" style="font-size: 3rem"></i>"#
            }
            Language::Go => r#"<i class="devicon-go-plain colored" style="font-size: 3rem"></i>"#,
            Language::C => r#"<i class="devicon-c-plain colored" style="font-size: 3rem"></i>"#,
            Language::OCaml => {
                r#"<i class="devicon-ocaml-plain colored" style="font-size: 3rem"></i>"#
            }
            Language::Bash => {
                r#"<i class="devicon-bash-plain colored" style="font-size: 3rem"></i>"#
            }
            Language::Clojure => {
                r#"<i class="devicon-clojure-plain colored" style="font-size: 3rem"></i>"#
            }
        }
    }

    pub fn to_tag(&self) -> String {
        render_text_tag(&self.to_string())
    }
}

// derive deserialize for Language
