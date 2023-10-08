use yew::prelude::*;

pub struct Wip;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub on_clicked: Callback<AttrValue>,
}

impl Component for Wip {
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
        html! {
            <div class="bg-black h-full animation-container">
                <div class="relative isolate px-6 pt-14 lg:px-8">
                    <div class="mx-auto max-w-2xl py-32 sm:py-48 lg:py-56 bg-opacity-50">
                        <div class="text-center">
                            <h1 class="text:lg md:text-4xl font-bold tracking-tight text-gray-100 sm:text-4xl">{"🚧Under Construction🚧"}</h1>
                            <p class="mt-6 text-sm md:text-lg leading-8 text-gray-300">{"This Page is Still Under Construction. Check back later!"}</p>
                            <div class="mt-10 flex items-center justify-center gap-x-6 mb-20">
                                <button
                                    onclick={ctx.props().on_clicked.reform(|_| "Home".into())}
                                    class="rounded-md disabled bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                                    {"Home"}
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

        }
    }
}
