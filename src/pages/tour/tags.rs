use yew::prelude::*;

pub enum Language {
    Haskell,
    Rust,
    C,
    Python,
    Clojure,
    Javascript,
    Go,
}
impl TagToHtml for Language {}

pub struct Category {
    pub name: String,
}
impl TagToHtml for Category {}

pub enum Tag {
    Language(Language),
    Category(Category),
}

impl Tag {
    pub fn to_html(&self) -> Html {
        match self {
            Tag::Language(lang) => match lang {
                Language::Haskell => lang.tag_with_color("Haskell", "purple"),
                Language::Rust => lang.tag_with_color("Rust", "orange"),
                Language::C => lang.tag_with_color("C", "blue"),
                Language::Python => lang.tag_with_color("Python", "yellow"),
                Language::Clojure => lang.tag_with_color("Clojure", "green"),
                Language::Javascript => lang.tag_with_color("Javascript", "yellow"),
                Language::Go => lang.tag_with_color("Go", "cyan"),
            },
            Tag::Category(cat) => match cat.name.as_str() {
                "Sorting" => cat.tag_with_color("Sorting", "blue"),
                c => cat.tag_with_color(c, "red"),
            },
        }
    }
}

pub trait TagToHtml {
    fn tag_with_color(&self, text: &str, color: &str) -> Html {
        let color_class = format!("inline-flex items-center rounded-md mr-2 px-2 py-1 text-xs font-medium ring-1 ring-inset bg-{}-500/20 text-{}-400 ring-{}-500/80", color, color, color);
        html! {
            <span class={color_class}>{text}</span>
        }
    }
}
