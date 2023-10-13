use crate::html_utils::make_tag;

use super::Language;
use yew::Html;
impl Language {
    pub fn icon(&self) -> Html {
        // https://devicon.dev/
        let icon = match self {
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
        };
        Html::from_html_unchecked(icon.into())
    }

    pub fn to_tag(&self) -> String {
        match self {
            Language::Haskell => make_tag("Haskell", "purple"),
            Language::Rust => make_tag("Rust", "orange"),
            Language::Python => make_tag("Python", "yellow"),
            Language::Go => make_tag("Go", "cyan"),
            Language::C => make_tag("C", "gray"),
            Language::OCaml => make_tag("OCaml", "blue"),
        }
    }
}

// derive deserialize for Language
