use regex::Regex;
use yew::Html;

struct TokenType {
    name: &'static str,
    pattern: Regex,
}

enum Either {
    Matched(String),
    NotMatched(String),
}

impl TokenType {
    fn new(name: &'static str, pattern: &'static str) -> Self {
        Self {
            name,
            pattern: Regex::new(pattern).unwrap(),
        }
    }
    fn parse(&self, either: Either) -> Either {
        match either {
            Either::Matched(token) => Either::Matched(token),
            Either::NotMatched(token) => match self.pattern.find(token.as_str()) {
                Some(_) => Either::Matched(format!("<span class='{}'>{}</span>", self.name, token)),
                None => Either::NotMatched(token),
            },
        }
    }
}

#[derive(Debug)]
pub struct Highlighter;

impl Highlighter {
    pub fn new() -> Self {
        Self
    }

    pub fn highlight(&self, input: String) -> Html {
        let token_types = vec![
            TokenType::new("comments", r"--.*\n"),
            TokenType::new("control", r"\b(if|else|case|of|then)\b"),
            TokenType::new("bind", r"\b(let|in|where|data|newtype|type)\b"),
            TokenType::new("op", r"->|\||<-|\.\.|::|:|=|@|~|\+\+|>|<"),
            TokenType::new("structs", r"[\[\](){}]"),
            TokenType::new("cls", r"[A-Z]\w+"),
        ];

        let re = Regex::new(r"(--.*\n|\s+|\[|\:+|\]|\(|\)|\{|\}|\w+|\S+)").unwrap();
        // let tokens: Vec<&str> =
        let res = re
            .captures_iter(input.as_str())
            .filter_map(|cap| cap.get(1))
            .map(|s| s.as_str())
            .map(|t| {
                // println!("{}", t);
                let mut maybe_match = Either::NotMatched(t.to_string());
                for token_type in &token_types {
                    maybe_match = token_type.parse(maybe_match);
                }
                match maybe_match {
                    Either::Matched(r) => r,
                    Either::NotMatched(r) => format!("<span class='{}'>{}</span>", "var", r),
                }
            })
            .collect::<String>();

        Html::from_html_unchecked(res.into())
    }
}
