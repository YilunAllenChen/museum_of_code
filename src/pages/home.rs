use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Home;

#[derive(Properties, PartialEq)]
pub struct HomeProps {}

impl Component for Home {
    type Message = ();
    type Properties = HomeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let formatted_haskell = Html::from_html_unchecked(
            include_str!("../artifacts/featured/quicksort_hs.html").into(),
        );

        let navigator = ctx.link().navigator().unwrap();
        let nav = |page| Callback::from(move |_| navigator.push(&page));

        let buttons = vec![
            ("Enter the Museum", Route::ExhibitionHall, "indigo"),
            ("About the Museum", Route::About, "yellow"),
            ("Help the Museum", Route::Contact, "green"),
        ]
        .into_iter()
        .map(|(text, route, color)| {
            html! {
                <button
                    onclick={nav.clone()(route)}
                    class={format!("rounded-md w-full md:w-fit bg-{}-600 my-2.5 px-5 py-3 text-sm font-semibold text-white hover:bg-{}-800", color, color)}
                >
                    {text}
                </button>
            }
        });

        html! {
            <div class="ease-in bg-black h-full">
                <div class="mx-6 sm:mx-10 md:mx-16 lg:mx-36 mt-24 lg:mt-40">
                    <div class="text-center">
                        <h1 class="text-4xl font-bold tracking-wider text-gray-100 sm:text-6xl font-mono">{"Museum of Code"}</h1>
                        <p class="mt-6 text-lg leading-8 text-gray-300">{"A curated exhibit of exquisite programming artifacts."}</p>
                        <div class="mt-10 flex flex-wrap items-center justify-center gap-x-6 mb-20">
                            {for buttons}
                        </div>
                    </div>
                    <div class="bg-gray-800 md:p-12 mx-auto max-w-xl text-gray-300 p-4 md:py-8 mb-8 rounded-md justify-left items-left">
                        <div class="mx-auto text-xs md:text-base font-mono text-center w-3/4 py-2 bg-gray-700 ring ring-8 ring-gray-800 rounded relative -top-8 md:-top-12 text-gray-300">{"  Season Spotlight  "}</div>
                        <pre>
                            {formatted_haskell}
                        </pre>
                    </div>
                </div>
            </div>

        }
    }
}
