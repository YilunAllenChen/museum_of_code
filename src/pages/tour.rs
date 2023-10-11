use std::vec;

use yew::prelude::*;

use crate::code::{Article, EntryStatus, Language};
use crate::Page;

use crate::html_utils::tags;

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
        let qs = include_str!("../artifacts/build/quicksort_hs.html");
        let rayon = include_str!("../artifacts/build/rayon_rs.html");
        let goroutines = include_str!("../artifacts/build/goroutines_go.html");
        let fac = include_str!("../artifacts/build/factorial_ml.html");
        let merge = include_str!("../artifacts/build/mergesort_hs.html");
        let uf_pc = include_str!("../artifacts/build/uf_pc_c.html");

        let articles: Html = vec![
            html! {
                <Article
                title={"Quick Sort"}
                language={Language::Haskell}
                status={EntryStatus::OnExhibit}
                tags={vec![
                    tags::Tag::LanguageTag(Language::Haskell),
                    tags::Tag::CategoryTag(tags::Category {name: "Sorting".to_string(),})]
                }
                code={qs.to_string()}
                desc={r#"Quick Sort is an extremely efficient sorting algorithm that uses divide and conquer strategy to sort a list of elements. The Haskell implementation is a very elegant example of the language's expressiveness."#.to_string()}
                />
            },
            html! {
                <Article
                title={"Rayon"}
                language={Language::Rust}
                status={EntryStatus::OnExhibit}
                tags={vec![
                    tags::Tag::LanguageTag(Language::Rust),
                    tags::Tag::CategoryTag(tags::Category {name: "Concurrency".to_string(),})]
                }
                code={rayon.to_string()}
                desc={r#"This short snippet demonstrates how, by leveraging the `rayon` crate, parallelism is made trivial - simply change `iter()` to `par_iter()`."#.to_string()}
                />
            },
            html! {
                <Article
                title={"Goroutines"}
                language={Language::Go}
                status={EntryStatus::OnExhibit}
                tags={vec![
                    tags::Tag::LanguageTag(Language::Go),
                    tags::Tag::CategoryTag(tags::Category {name: "Concurrency".to_string(),})]
                }
                code={goroutines.to_string()}
                desc={r#"This short snippet demonstrates the use of goroutines in Go. This is a great example of how Go's concurrency primitives make it easy to write concurrent programs.ms."#.to_string()}
                />
            },
            html! {
                <Article
                title={"Fibonacci"}
                language={Language::Python}
                status={EntryStatus::OnExhibit}
                tags={vec![ tags::Tag::LanguageTag(Language::Python) ]}
                code={goroutines.to_string()}
                desc={r#"This simple function computes the nth Fibonacci number. It demonstrates the incredible readability of the language."#.to_string()}
                />
            },
            html! {
                <Article
                title={"Factorial"}
                language={Language::OCaml}
                status={EntryStatus::OnExhibit}
                tags={vec![
                    tags::Tag::LanguageTag(Language::OCaml),
                ]}
                code={fac}
                desc={r#"This is one of the first programs OCaml programmers write. It is a simple recursive function that computes the factorial of a number. It demonstrates the syntax of OCaml, as well as the use of pattern matching and recursion."#.to_string()}
                />
            },
            html! {
                <Article
                title={"Merge Sort"}
                language={Language::Haskell}
                status={EntryStatus::OnExhibit}
                tags={vec![
                    tags::Tag::LanguageTag(Language::Haskell),
                ]}
                code={merge}
                desc={r#"This program implements the merge sort algorithm. It leverages Haskell's pattern matching to implement the merge function in a concise and expressive manner."#.to_string()}
                />
            },
            html! {
                <Article
                title={"Union Find with Path Compression"}
                language={Language::C}
                status={EntryStatus::OnExhibit}
                tags={vec![
                    tags::Tag::LanguageTag(Language::C),
                ]}
                code={uf_pc}
                desc={r#"Union Find is a data structure that allows for efficient union and find operations on disjoint sets. This implementation uses path compression to achieve amortized constant time find operations."#.to_string()}
                />
            },
            html! {
                <Article
                title={"Dijkstra's Algorithm"}
                language={Language::Python}
                status={EntryStatus::StagedForExhibit}
                tags={vec![
                    tags::Tag::LanguageTag(Language::Python),
                ]}
                code={r#""#.to_string()}
                desc={r#""#.to_string()}
                />
            },
            html! {
                <Article
                title={"Linux Kernel"}
                language={Language::C}
                status={EntryStatus::Maintenance}
                tags={vec![
                    tags::Tag::LanguageTag(Language::C),
                ]}
                code={"".to_string()}
                desc={"".to_string()}
                />
            },
        ]
        .into_iter()
        .collect();

        html! {
            <div class="bg-black h-full">
                <ul role="list" class="text-white px-4 md:px-40 md:py-10 divide-y divide-gray-800">
                    {articles}
                </ul>
            </div>
        }
    }
}
