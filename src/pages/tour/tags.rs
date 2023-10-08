use yew::prelude::*;

pub enum Language {
    Haskell,
    Rust,
    Fortran,
}
impl TagToHtml for Language {}

pub struct Category {
    pub name: String
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
                Language::Haskell => lang.red("Haskell"),
                Language::Rust => lang.blue("Rust"),
                Language::Fortran => lang.indigo("Fortran"),
            }
            Tag::Category(cat) => match cat.name.as_str() {
                "Sorting" => cat.yellow("Sorting"),
                "Searching" => cat.green("Searching"),
                "Graphs" => cat.purple("Graphs"),
                "Hello World" => cat.pink("Hello World"),
                _ => cat.normal(&cat.name),
            }
        }
    }
}

static BASE_CLASS: &str = "inline-flex items-center rounded-md mx-1 px-2 py-1 text-xs font-medium ring-1 ring-inset {}";
pub trait TagToHtml {

    fn normal(&self, text: &str) -> Html {
        html! {
            <span class={format!("{}{}", BASE_CLASS, "bg-gray-500/20 text-gray-400 ring-gray-500/80")}>{text}</span>
        }
    }

    fn red(&self, text: &str) -> Html {
        html! {
            <span class={format!("{}{}", BASE_CLASS, "bg-red-500/20 text-red-400 ring-red-500/80")}>{text}</span>
        }
    }

    fn yellow(&self, text: &str) -> Html {
        html! {
            <span class={format!("{}{}", BASE_CLASS, "bg-yellow-500/20 text-yellow-400 ring-yellow-500/80")}>{text}</span>
        }
    }

    fn green(&self, text: &str) -> Html {
        html! {
            <span class={format!("{}{}", BASE_CLASS, "bg-green-500/20 text-green-400 ring-green-500/80")}>{text}</span>
        }
    }

    fn blue(&self, text: &str) -> Html {
        html! {
            <span class={format!("{}{}", BASE_CLASS, "bg-blue-500/20 text-blue-400 ring-blue-500/80")}>{text}</span>
        }
    }

    fn indigo(&self, text: &str) -> Html {
        html! {
            <span class={format!("{}{}", BASE_CLASS, "bg-indigo-500/20 text-indigo-400 ring-indigo-500/80")}>{text}</span>
        }
    }

    fn purple(&self, text: &str) -> Html {
        html! {
            <span class={format!("{}{}", BASE_CLASS, "bg-purple-500/20 text-purple-400 ring-purple-500/80")}>{text}</span>
        }
    }

    fn pink(&self, text: &str) -> Html {
        html! {
            <span class={format!("{}{}", BASE_CLASS, "bg-pink-500/20 text-pink-400 ring-pink-500/80")}>{text}</span>
        }
    }

    fn gray(&self, text: &str) -> Html {
        html! {
            <span class={format!("{}{}", BASE_CLASS, "bg-gray-500/20 text-gray-400 ring-gray-500/80")}>{text}</span>
        }
    }
}