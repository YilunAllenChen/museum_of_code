use log::warn;
use regex::Regex;
use yew::Html;

use crate::code::highlighter::Highlighter;
use crate::html_utils::tags::HTMLTag;

use super::highlighter::TokenType;

#[derive(Clone, PartialEq, Debug)]
pub enum Language {
    Haskell,
    Rust,
    Python,
    Go,
    C,
    OCaml,
}
impl HTMLTag for Language {}

impl Language {
    pub fn to_html(&self) -> Html {
        match self {
            Language::Haskell => self.tag("Haskell", "purple"),
            Language::Rust => self.tag("Rust", "orange"),
            Language::Python => self.tag("Python", "yellow"),
            Language::Go => self.tag("Go", "cyan"),
            Language::C => self.tag("C", "gray"),
            Language::OCaml => self.tag("OCaml", "blue"),
        }
    }

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

    pub fn highlight(&self, code: String) -> Html {
        let highlighter = Highlighter::new();
        match self {
            Language::Haskell => highlighter.highlight(
                code,
                Regex::new(r"(--.*|\n|\.|\s+|\[|\:+|\]|\(|\)|\{|\}|\w+|\S+)").unwrap(),
                vec![
                    TokenType::new("comments", r"--.*"),
                    TokenType::new("control", r"\b(if|else|case|of|then)\b"),
                    TokenType::new("bind", r"\b(let|in|where|data|newtype|type)\b"),
                    TokenType::new("op", r"->|\||<-|\.\.|::|:|=|@|~|\+\+|>|<"),
                    TokenType::new("structs", r"[\[\](){}]"),
                    TokenType::new("cls", r"[A-Z]\w+"),
                ],
            ),
            Language::Rust => highlighter.highlight(code,
                Regex::new(r"(\/\/.*|\n|\.|\s+|\[|\:+|\]|\(|\)|\{|\}|\w+|\S+)").unwrap(),
                vec![
                    TokenType::new("comments", r"\/\/.*"),
                    TokenType::new(
                        "control",
                        r"\b(if|else|match|for|while|in|return|fn|struct|enum|impl|trait|use|pub|mod|as|from|break|continue)\b",
                    ),
                    TokenType::new("op", r"->|\||<-|\.\.|::|:|=|@|~|\+\+|>|<"),
                    TokenType::new("structs", r"[\[\](){}]"),
                    TokenType::new("cls", r"[A-Z]\w+"),
                ]
            ),
            Language::Python => highlighter.highlight(code,
            Regex::new(r"(#.*\n|\n|\.|\s+|\[|\:+|\]|\(|\)|\{|\}|\w+|\S+)").unwrap(),
            vec![
                TokenType::new("comments", r"#.*"),
                TokenType::new(
                    "control",
                    r"\b(if|else|elif|for|while|in|return|def|class|with|as|from|import|pass|break|continue)\b",
                ),
                TokenType::new("op", r"->|\||<-|\.\.|::|:|=|@|~|\+\+|>|<"),
                TokenType::new("structs", r"[\[\](){}]"),
                TokenType::new("cls", r"[A-Z]\w+"),
            ]),
            _ => {
                warn!(
                    "Highlighting for {:?} not implemented yet; using generic parser",
                    self
                );
                highlighter.highlight(code,
                    Regex::new(r"(--.*|\/\/.*|\n|\.|\s+|\[|\:+|\]|\(|\)|\{|\}|\w+|\S+)").unwrap(),
                    vec![
                        TokenType::new("comments", r"\/\/.*"),
                        TokenType::new(
                            "control",
                            r"\b(if|else|match|for|while|in|return|fn|struct|enum|impl|trait|use|pub|mod|as|from|break|continue)\b",
                        ),
                        TokenType::new("op", r"->|\||<-|\.\.|::|:|=|@|~|\+\+|>|<"),
                        TokenType::new("structs", r"[\[\](){}]"),
                        TokenType::new("cls", r"[A-Z]\w+"),
                    ]
                )
            }
        }
    }
}
