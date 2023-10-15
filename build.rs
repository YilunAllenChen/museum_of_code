use std::fs;
use std::path::Path;
extern crate regex;
use regex::Regex;

include!("src/code/raw_artifact.rs");

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

pub fn highlight_html(
    input: String,
    tokenizer_pattern: Regex,
    token_types: Vec<TokenType>,
) -> String {
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
        Language::Haskell => highlight_html(
            code,
            Regex::new(r"(--.*\n|\n|\.|\s+|\[|\:+|\]|\(|\)|\{|\}|\w+|\S+)").unwrap(),
            vec![
                TokenType::new("comments", r"--.*\n"),
                TokenType::new("control", r"\b(if|else|case|of|then)\b"),
                TokenType::new("bind", r"\b(let|in|where|data|newtype|type)\b"),
                TokenType::new("op", r"->|\||<-|\.\.|::|:|=|@|~|\+\+|>|<"),
                TokenType::new("structs", r"[\[\](){}]"),
                TokenType::new("cls", r"[A-Z]\w+"),
            ],
        ),
        Language::Rust => highlight_html(
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
        Language::Python => highlight_html(
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
            highlight_html(
                code,
                Regex::new(r"(--.*\n|\/\/.*|\n|\.|\s+|\[|\:+|\]|\(|\)|\{|\}|\w+|\S+)").unwrap(),
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
    let path_pattern = Regex::new(r"src/artifacts.*\.yaml").unwrap();

    let articles: Vec<RawArticle> = fs::read_dir("src/artifacts")
        .unwrap()
        .map(|f| f.unwrap())
        .filter(|f| f.metadata().unwrap().is_file())
        .map(|f| f.path())
        .map(|path| {
            let path_str = path.to_str().unwrap();
            match path_pattern.captures(path.to_str().unwrap()) {
                Some(_) => path_str.to_string(),
                None => {
                    panic!("Invalid file detected under artifact: {}", path_str);
                }
            }
        })
        .map(|path| {
            let raw_artifact: RawArticle = serde_yaml::from_str(
                fs::read_to_string(path)
                    .expect("Failed to read the file")
                    .as_str(),
            )
            .unwrap();

            let lang: Language = serde_yaml::from_str(raw_artifact.language.as_str()).unwrap();
            let highlighted_code = highlight(lang, raw_artifact.code);
            RawArticle {
                title: raw_artifact.title,
                language: raw_artifact.language,
                status: raw_artifact.status,
                tags: raw_artifact.tags,
                code: highlighted_code,
                desc: raw_artifact.desc,
            }
            // make dir if not exists
        })
        .collect();

    // build is millis since epoch
    let meta = MetaYaml {
        build: chrono::Utc::now().timestamp_millis().to_string(),
    };

    let built_yaml = BuiltYaml {
        artifacts: articles,
        meta,
    };

    let output_path = Path::new("src/artifacts/build/compiled.yaml");
    let parent = output_path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(parent).expect("Failed to create the dir");
    }
    match fs::write(output_path, serde_yaml::to_string(&built_yaml).unwrap()) {
        Ok(_) => println!("Successfully wrote to {}", output_path.display()),
        Err(e) => panic!("Failed to write to {}: {}", output_path.display(), e),
    }
}
