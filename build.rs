use std::fs;
use std::path::Path;
extern crate regex;

use regex::Regex;

include!("src/code/lang.rs");

pub struct TokenType {
    name: &'static str,
    pattern: Regex,
}

#[derive(Debug)]
enum Either {
    Matched(String),
    NotMatched(String),
}

impl TokenType {
    pub fn new(name: &'static str, pattern: &'static str) -> Self {
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

pub fn tokenize(input: String, tokenizer_pattern: Regex, token_types: Vec<TokenType>) -> String {
    // let tokens: Vec<&str> =
    tokenizer_pattern
        .captures_iter(input.as_str())
        .filter_map(|cap| cap.get(1))
        .map(|s| s.as_str())
        .map(|t| {
            let mut maybe_match = Either::NotMatched(t.to_string());
            for token_type in &token_types {
                maybe_match = token_type.parse(maybe_match);
            }
            match maybe_match {
                Either::Matched(r) => r,
                Either::NotMatched(r) => format!("<span class='{}'>{}</span>", "var", r),
            }
        })
        .collect::<String>()
}

pub fn highlight(lang: Language, code: String) -> String {
    match lang {
        Language::Haskell => tokenize(
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
        Language::Rust => tokenize(
            code,
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
            ],
        ),
        Language::Python => tokenize(
            code,
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
            ],
        ),
        _ => {
            println!(
                "Highlighting for {:?} not implemented yet; using generic parser",
                lang
            );
            tokenize(
                code,
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
                ],
            )
        }
    }
}

fn main() {
    let path_pattern = Regex::new(r"src/artifacts(.*)\.(.*)\.artifact").unwrap();

    fs::read_dir("src/artifacts")
        .unwrap()
        .map(|f| f.unwrap())
        .filter(|f| f.metadata().unwrap().is_file())
        .map(|f| f.path())
        .for_each(|path| {
            let path_str = path.to_str().unwrap();
            println!("Processing {:?}", path_str);
            let captures = path_pattern.captures(path_str).unwrap();

            let (name, ext) = (
                captures.get(1).unwrap().as_str(),
                captures.get(2).unwrap().as_str(),
            );
            let content = fs::read_to_string(&path).expect("Failed to read the file");
            let output_str = format!("src/artifacts/build/{}_{}.html", name, ext);
            let output_path = Path::new(&output_str);
            let processed = match ext {
                "rs" => highlight(Language::Rust, content),
                "hs" => highlight(Language::Haskell, content),
                "py" => highlight(Language::Python, content),
                "go" => highlight(Language::Go, content),
                "ml" => highlight(Language::OCaml, content),
                "c" => highlight(Language::C, content),
                _ => {
                    panic!("Unknown extension: {}", ext)
                }
            };

            fs::write(output_path, processed).expect("Failed to write the processed content");
        });
}
