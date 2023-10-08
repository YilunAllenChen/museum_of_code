use yew::prelude::*;

use super::tags::{Language, Tag};

pub enum EntryStatus {
    OnExhibit,
    StagedForExhibit,
    Maintenance,
}

pub struct Article {
    title: String,
    date: String,
    status: EntryStatus,
    tags: Vec<Tag>,
    icon: &'static str, // https://devicon.dev/
}

impl Article {
    pub fn haskell(title: &str, date: &str, status: EntryStatus, mut tags: Vec<Tag>) -> Self {
        tags.push(Tag::Language(Language::Haskell));
        Self {
            title: title.to_string(),
            date: date.to_string(),
            status,
            tags,
            icon: r#"<i class="devicon-haskell-plain colored" style="font-size: 3rem"></i>"#,
        }
    }

    pub fn rust(title: &str, date: &str, status: EntryStatus, mut tags: Vec<Tag>) -> Self {
        tags.push(Tag::Language(Language::Rust));
        Self {
            title: title.to_string(),
            date: date.to_string(),
            status,
            tags,
            icon: r#"<i class="devicon-rust-plain" style="font-size: 3rem"></i>"#,
        }
    }

    pub fn python(title: &str, date: &str, status: EntryStatus, mut tags: Vec<Tag>) -> Self {
        tags.push(Tag::Language(Language::Python));
        Self {
            title: title.to_string(),
            date: date.to_string(),
            status,
            tags,
            icon: r#"<i class="devicon-python-plain colored" style="font-size: 3rem"></i>"#,
        }
    }

    pub fn javascript(title: &str, date: &str, status: EntryStatus, mut tags: Vec<Tag>) -> Self {
        tags.push(Tag::Language(Language::Javascript));
        Self {
            title: title.to_string(),
            date: date.to_string(),
            status,
            tags,
            icon: r#"<i class="devicon-javascript-plain colored" style="font-size: 3rem"></i>"#,
        }
    }

    pub fn c(title: &str, date: &str, status: EntryStatus, mut tags: Vec<Tag>) -> Self {
        tags.push(Tag::Language(Language::C));
        Self {
            title: title.to_string(),
            date: date.to_string(),
            status,
            tags,
            icon: r#"<i class="devicon-c-plain colored" style="font-size: 3rem"></i>"#,
        }
    }

    pub fn go(title: &str, date: &str, status: EntryStatus, mut tags: Vec<Tag>) -> Self {
        tags.push(Tag::Language(Language::Go));
        Self {
            title: title.to_string(),
            date: date.to_string(),
            status,
            tags,
            icon: r#"<i class="devicon-go-plain colored" style="font-size: 3rem"></i>"#,
        }
    }

    pub fn clojure(title: &str, date: &str, status: EntryStatus, mut tags: Vec<Tag>) -> Self {
        tags.push(Tag::Language(Language::Clojure));
        Self {
            title: title.to_string(),
            date: date.to_string(),
            status,
            tags,
            icon: r#"<i class="devicon-clojure-plain colored" style="font-size: 3rem"></i>"#,
        }
    }

    pub fn to_html(&self) -> Html {
        let dot_and_text = match self.status {
            EntryStatus::OnExhibit => {
                html! {
                    <div class="mt-1 flex items-center gap-x-1.5">
                      <p class="text-xs leading-5 text-gray-200">{"On Exhibit"}</p>
                      <div class="flex-none rounded-full bg-emerald-500/20 p-1">
                        <div class="h-1.5 w-1.5 rounded-full bg-emerald-500"></div>
                      </div>
                    </div>
                }
            }
            EntryStatus::StagedForExhibit => {
                html! {
                    <div class="mt-1 flex items-center gap-x-1.5">
                      <p class="text-xs leading-5 text-gray-200">{"Staged"}</p>
                      <div class="flex-none rounded-full bg-yellow-500/20 p-1">
                        <div class="h-1.5 w-1.5 rounded-full bg-yellow-500"></div>
                      </div>
                    </div>
                }
            }
            EntryStatus::Maintenance => {
                html! {
                    <div class="mt-1 flex items-center gap-x-1.5">
                      <p class="text-xs leading-5 text-gray-200">{"Maintenance"}</p>
                      <div class="flex-none rounded-full bg-red-500/20 p-1">
                        <div class="h-1.5 w-1.5 rounded-full bg-red-500"></div>
                      </div>
                    </div>
                }
            }
        };

        let tags: Html = self.tags.iter().map(|tag| tag.to_html()).collect();

        html! {
            <li class="flex justify-between gap-x-6 py-5">
                <div class="flex min-w-0 gap-x-4">
                    {Html::from_html_unchecked(self.icon.into())}
                    <div class="min-w-0 flex-auto">
                    <p class="text-sm font-semibold leading-6 text-gray-100">{self.title.clone()}</p>
                    <p class="mt-1 truncate text-xs leading-5 text-gray-300">{tags}</p>
                    </div>
                </div>
                <div class="shrink-0 flex flex-col items-end">
                    <p class="text-sm leading-6 text-gray-100">{self.date.clone()}</p>
                    {dot_and_text}
                </div>
            </li>
        }
    }
}
