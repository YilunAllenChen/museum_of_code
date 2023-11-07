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

impl Language {
    pub fn to_string(&self) -> String {
        match self {
            Language::Haskell => "Haskell".to_string(),
            Language::Rust => "Rust".to_string(),
            Language::Python => "Python".to_string(),
            Language::Go => "Go".to_string(),
            Language::C => "C".to_string(),
            Language::OCaml => "OCaml".to_string(),
            Language::Bash => "Bash".to_string(),
            Language::Clojure => "Clojure".to_string(),
        }
    }

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
