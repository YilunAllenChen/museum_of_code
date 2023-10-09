use yew::prelude::*;

use crate::code::Language;

#[derive(Clone, PartialEq)]
pub struct Category {
    pub name: String,
}
impl HTMLTag for Category {}

#[derive(Clone, PartialEq)]
pub enum Tag {
    LanguageTag(Language),
    CategoryTag(Category),
}

impl Tag {
    pub fn to_html(&self) -> Html {
        match self {
            Tag::LanguageTag(lang) => lang.to_html(),
            Tag::CategoryTag(cat) => match cat.name.as_str() {
                "Sorting" => cat.tag("Sorting", "blue"),
                "Concurrency" => cat.tag("Concurrency", "green"),
                c => cat.tag(c, "gray"),
            },
        }
    }
}

pub trait HTMLTag {
    fn tag(&self, text: &str, color: &str) -> Html {
        let color_class = format!("inline-flex items-center rounded-md mr-2 px-2 py-1 text-xs font-medium ring-1 ring-inset bg-{}-500/20 text-{}-400 ring-{}-500/80", color, color, color);
        html! {
            <span class={color_class}>{text}</span>
        }
    }
}
