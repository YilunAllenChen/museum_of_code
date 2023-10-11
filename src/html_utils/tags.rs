use crate::code::Language;

#[derive(Clone, PartialEq)]
pub struct Category {
    pub name: String,
}

#[derive(Clone, PartialEq)]
pub enum Tag {
    LanguageTag(Language),
    CategoryTag(Category),
}

impl Tag {
    fn make_tag(text: &str, color: &str) -> String {
        let color_class = format!("inline-flex items-center rounded-md mr-2 px-2 py-1 text-xs font-medium ring-1 ring-inset bg-{}-500/20 text-{}-400 ring-{}-500/80", color, color, color);
        format!(r#"<span class="{}">{}</span>"#, color_class, text).into()
    }

    pub fn to_html(&self) -> String {
        match self {
            Tag::LanguageTag(lang) => match lang {
                Language::Haskell => Self::make_tag("Haskell", "purple"),
                Language::Rust => Self::make_tag("Rust", "orange"),
                Language::Python => Self::make_tag("Python", "yellow"),
                Language::Go => Self::make_tag("Go", "cyan"),
                Language::C => Self::make_tag("C", "gray"),
                Language::OCaml => Self::make_tag("OCaml", "blue"),
            },
            Tag::CategoryTag(cat) => match cat.name.as_str() {
                "Sorting" => Self::make_tag("Sorting", "blue"),
                "Concurrency" => Self::make_tag("Concurrency", "green"),
                c => Self::make_tag(c, "gray"),
            },
        }
    }
}
