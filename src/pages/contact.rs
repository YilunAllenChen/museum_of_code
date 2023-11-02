use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::Route;

pub struct Contact;

#[derive(Properties, PartialEq)]
pub struct ContactProps {}

impl Component for Contact {
    type Message = ();
    type Properties = ContactProps;

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
        let navigator = ctx.link().navigator().unwrap();
        let nav = |page| Callback::from(move |_| navigator.push(&page));

        html! {
            <div class="ease-in bg-black h-full">
                <figure class="m-4 md:mx-auto md:my-16 md:w-3/4 lg:w-1/2 pb-12 bg-slate-800 rounded-xl px-2 md:px-8 bg-slate-800">
                    <div class="mt-12 pt-16 self-center text-left divide-y divide-gray-500 space-y-8 md:space-y-8">
                        <p class="text-2xl text-center md:text-4xl font-bold tracking-tight text-gray-100">{"Getting Involved"}</p>
                        <div class="text-center px-8 divide-y divide-gray-700 space-y-8">
                            <div class="text-center flex flex-col items-center justify-center gap-x-6">
                                <p class="pt-1 my-4 text-gray-200 text-xs md:text-sm">{"⚒️ Want to help build the museum? Let's get in touch!"}</p>
                                <a class="rounded-md w-1/2 bg-blue-700 px-3.5 py-2.5 text-base text-white shadow-sm hover:bg-blue-500"
                                    href="mailto:allenchenyilun1999@gmail.com"
                                >{"Email Us!"}</a>
                                <p class="pt-1 text-gray-400 text-xs md:text-sm">{"@allenchenyilun1999@gmail.com"}</p>
                            </div>
                            <div class="flex flex-col items-center justify-center gap-x-6 mb-20">
                                <p class="pt-1 my-4 text-gray-200 text-xs md:text-sm">{"🤝 Want to put your name/logo in the museum?"}</p>
                                <a class="rounded-md w-1/2 bg-emerald-700 px-3.5 py-2.5 text-base text-white shadow-sm hover:bg-emerald-500"
                                    href="https://github.com/sponsors/YilunAllenChen"
                                >{"Sponsor Us!"}</a>
                                <p class="pt-1 text-gray-400 text-xs md:text-sm">{"via GitHub Sponsor"}</p>
                            </div>
                            <div class="flex justify-end md:justify-end">
                                <button
                                    onclick={nav(Route::Home)}
                                    class="mt-8 rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm text-white shadow-sm hover:bg-indigo-500">
                                    {"Go Back"}
                                </button>
                            </div>
                        </div>
                    </div>
                </figure>
            </div>
        }
    }
}
