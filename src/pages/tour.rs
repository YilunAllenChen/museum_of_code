use std::vec;

use yew::prelude::*;

use crate::Page;

mod articles;
mod tags;

use articles::{Article, EntryStatus};
use tags::Tag;

pub struct Tour;

#[derive(Properties, PartialEq)]
pub struct TourProps {
    pub on_clicked: Callback<Page>,
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
            Article::haskell(
                "Quick Sort",
                "2023/10/08",
                EntryStatus::OnExhibit,
                vec![Tag::Category(tags::Category {
                    name: "Sorting".to_string(),
                })],
            ),
            Article::haskell("Dijkstra", "2023/10/08", EntryStatus::Maintenance, vec![]),
            Article::rust("Rayon", "2023/10/08", EntryStatus::StagedForExhibit, vec![]),
            Article::go(
                "Goroutine",
                "2023/10/08",
                EntryStatus::OnExhibit,
                vec![Tag::Category(tags::Category {
                    name: "Concurrency".to_string(),
                })],
            ),
            Article::python(
                "Web Server",
                "2023/10/08",
                EntryStatus::StagedForExhibit,
                vec![],
            ),
            Article::clojure("DFS", "2023/10/08", EntryStatus::OnExhibit, vec![]),
            Article::javascript(
                "Async/Await",
                "2023/10/08",
                EntryStatus::Maintenance,
                vec![Tag::Category(tags::Category {
                    name: "Concurrency".to_string(),
                })],
            ),
            Article::c(
                "Kernel",
                "2023/10/08",
                EntryStatus::StagedForExhibit,
                vec![],
            ),
        ]
        .into_iter()
        .map(|entry| entry.to_html())
        .collect();

        html! {
            <div class="bg-black h-full">
                <ul role="list" class="text-white px-4 divide-y divide-gray-800">
                    {articles}
                </ul>
            </div>
        }
    }
}
