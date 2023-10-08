use std::vec;

use yew::prelude::*;

use crate::Page;

mod tags;

use tags::Tag;

pub struct Tour;

#[derive(Properties, PartialEq)]
pub struct TourProps {
    pub on_clicked: Callback<Page>,
}

pub enum EntryStatus {
    OnExhibit,
    StagedForExhibit,
    Maintenance,
}

struct Entry {
    title: String,
    date: String,
    status: EntryStatus,
    tags: Vec<Tag>,
    image: String,
}

impl Entry {
    pub fn haskell(title: &str, date: &str, status: EntryStatus, mut tags: Vec<Tag>) -> Self {
        tags.push(Tag::Language(tags::Language::Haskell));
        Self {
            title: title.to_string(),
            date: date.to_string(),
            status,
            tags,
            image: "https://cdn-icons-png.flaticon.com/512/5968/5968214.png".to_string(),
        }
    }

    pub fn rust(title: &str, date: &str, status: EntryStatus, mut tags: Vec<Tag>) -> Self {
        tags.push(Tag::Language(tags::Language::Rust));
        Self {
            title: title.to_string(),
            date: date.to_string(),
            status,
            tags,
            image:
                "https://www.fpcomplete.com/wp-content/uploads/2023/04/rust-logo-512x512_white.png"
                    .to_string(),
        }
    }

    pub fn fortran(title: &str, date: &str, status: EntryStatus, mut tags: Vec<Tag>) -> Self {
        tags.push(Tag::Language(tags::Language::Fortran));
        Self {
            title: title.to_string(),
            date: date.to_string(),
            status,
            tags,
            image: "https://upload.wikimedia.org/wikipedia/commons/thumb/b/b8/Fortran_logo.svg/255px-Fortran_logo.svg.png".to_string(),
        }
    }

    pub fn to_html(&self) -> Html {
        let dot_and_text = match self.status {
            EntryStatus::OnExhibit => {
                html! {
                    <div class="mt-1 flex items-center gap-x-1.5">
                      <div class="flex-none rounded-full bg-emerald-500/20 p-1">
                        <div class="h-1.5 w-1.5 rounded-full bg-emerald-500"></div>
                      </div>
                      <p class="text-xs leading-5 text-gray-200">{"On Exhibit"}</p>
                    </div>
                }
            }
            EntryStatus::StagedForExhibit => {
                html! {
                    <div class="mt-1 flex items-center gap-x-1.5">
                      <div class="flex-none rounded-full bg-yellow-500/20 p-1">
                        <div class="h-1.5 w-1.5 rounded-full bg-yellow-500"></div>
                      </div>
                      <p class="text-xs leading-5 text-gray-200">{"Staged"}</p>
                    </div>
                }
            }
            EntryStatus::Maintenance => {
                html! {
                    <div class="mt-1 flex items-center gap-x-1.5">
                      <div class="flex-none rounded-full bg-red-500/20 p-1">
                        <div class="h-1.5 w-1.5 rounded-full bg-red-500"></div>
                      </div>
                      <p class="text-xs leading-5 text-gray-200">{"Maintenance"}</p>
                    </div>
                }
            }
        };

        let tags: Html = self.tags.iter().map(|tag| tag.to_html()).collect();

        html! {
            <li class="flex justify-between gap-x-6 py-5">
                <div class="flex min-w-0 gap-x-4">
                    <img class="h-12 w-12 flex-none" src={self.image.clone()} alt="" />
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

impl Component for Tour {
    type Message = ();
    type Properties = TourProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let articles: Html = vec![
            Entry::haskell(
                "Quick Sort",
                "2023/10/08",
                EntryStatus::OnExhibit,
                vec![Tag::Category(tags::Category {
                    name: "Sorting".to_string(),
                })],
            ),
            Entry::haskell("Dijkstra", "2023/10/08", EntryStatus::Maintenance, vec![]),
            Entry::rust("Rayon", "2023/10/08", EntryStatus::StagedForExhibit, vec![]),
            Entry::fortran("Hello World", "2023/10/08", EntryStatus::OnExhibit, vec![]),
        ]
        .into_iter()
        .map(|entry| entry.to_html())
        .collect();

        html! {
            <div class="bg-black h-full">
                <ul role="list" class="text-white px-8 divide-y divide-gray-500">
                    {articles}
                </ul>
            </div>
        }
    }
}
