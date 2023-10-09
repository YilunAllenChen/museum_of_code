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
        let articles: Html = vec![html! {
            <Article
            title={"Quick Sort"}
            language={Language::Haskell}
            status={EntryStatus::OnExhibit}
            tags={vec![
                tags::Tag::LanguageTag(Language::Haskell),
                tags::Tag::CategoryTag(tags::Category {name: "Sorting".to_string(),})]
            }
            code={r#"
qs :: Ord a => [a] -> [a]
qs [] = []
qs (x:xs) = qs lower ++ [x] ++ qs higher
  where
    lower = filter (<= x)
    higher = filter (> x)
"#.to_string().trim().to_string()}
            desc={r#"
Quick Sort is an extremely efficient sorting algorithm that uses divide and conquer strategy to sort a list of elements.

The Haskell implementation is a very elegant example of the language's expressiveness.
"#.to_string().trim().to_string()}
            />
        },
        html!{
            <Article
            title={"Rayon"}
            language={Language::Rust}
            status={EntryStatus::OnExhibit}
            tags={vec![
                tags::Tag::LanguageTag(Language::Rust),
                tags::Tag::CategoryTag(tags::Category {name: "concurrency".to_string(),})]
            }
            code={r#"
use rayon::prelude::*;

fn count_len(list: &[&str]) -> usize { 
  list
    // .iter()
    .par_iter()
    .map(|word| word.len())
    .sum() 
}
"#.to_string().trim().to_string()}
            desc={r#"
This function counts the length of all the words in a list.

By leveraging the `rayon` crate, parallelism is made trivial - simply change `iter()` to `par_iter()`.

This is a great example of how Rust's traits and generics allow for easy extension of existing code.
"#.to_string().trim().to_string()}
            />
        },

        html!{
            <Article
            title={"Fibonacci"}
            language={Language::Python}
            status={EntryStatus::OnExhibit}
            tags={vec![
                tags::Tag::LanguageTag(Language::Python),
                tags::Tag::CategoryTag(tags::Category {name: "Fibonacci".to_string(),})]
            }
            code={r#"
def fib(n):
    if n <= 1:
        return n
    return fib(n-1) + fib(n-2)
"#.to_string().trim().to_string()}
            desc={r#"
The function computes the nth Fibonacci number.

This is a great example of how Python's simplicity allows for easy implementation of complex algorithms.
"#.to_string().trim().to_string()}
            />
        },

            html!{
            <Article
            title="Goroutines"
            language={Language::Go}
            status={EntryStatus::OnExhibit}
            tags={vec![
                tags::Tag::LanguageTag(Language::Go),
                tags::Tag::CategoryTag(tags::Category {name: "concurrency".to_string(),})]
            }
            code={r#"
package main

import (
    "fmt"
    "time"
)

func main() {
    go say("world")
    say("hello")
}

func say(s string) {
    for i := 0; i < 5; i++ {
        time.Sleep(100 * time.Millisecond)
        fmt.Println(s)
    }
}
"#.to_string().trim().to_string()}
            desc={r#"
This program prints "hello" and "world" 5 times each, but concurrently.

This is a great example of how Go's concurrency primitives make it easy to write concurrent programs.

"#.to_string().trim().to_string()}
            />
        },

        html!{
            <Article
            title={"Factorial"}
            language={Language::OCaml}
            status={EntryStatus::OnExhibit}
            tags={vec![
                tags::Tag::LanguageTag(Language::OCaml),
            ]}
            code={r#"
let rec factorial n =
  if n <= 1 then 1
  else n * factorial (n - 1)
            "#.to_string()}
            desc={r#"
The function computes the factorial of a number.

This is a great example of how OCaml's simplicity allows for easy implementation of complex algorithms."#.to_string()}
            />
        },

        html!{
            <Article
            title={"Pointers"}
            language={Language::C}
            status={EntryStatus::StagedForExhibit}
            tags={vec![
                tags::Tag::LanguageTag(Language::C),
            ]}
            code={r#"
int main() {
    int x = 5;
    int *y = &x;
    printf("%d\n", *y);
}
            "#.to_string()}
            desc={r#"
This program prints the value of `x` by dereferencing the pointer `y`.

This demonstrates the low level control that the language has given to the programmer.
"#.to_string()}
            />
        },

        html!{
            <Article
            title={"Linux Kernel"}
            language={Language::C}
            status={EntryStatus::Maintenance}
            tags={vec![
                tags::Tag::LanguageTag(Language::C),
                tags::Tag::CategoryTag(tags::Category {name: "Sorting".to_string(),})]
            }
            code={"".to_string()}
            desc={"".to_string()}
            />
        },

        ]
        .into_iter()
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
