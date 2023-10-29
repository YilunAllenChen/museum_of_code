use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::Route;

pub struct About;

#[derive(Properties, PartialEq)]
pub struct AboutProps {}

impl Component for About {
    type Message = ();
    type Properties = AboutProps;

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

        let title_cls = "text-lg md:text-xl lg:text-4xl py-4 font-semibold text-gray-100";
        let text_cls = "py-2 text-gray-300 text-sm md:text-base leading-relaxed";

        html! {
        <div class="ease-in bg-black h-full">
            <figure class="m-4 md:mx-auto md:my-16 md:w-3/4 lg:w-1/2 lg:mx-40 pb-12 bg-slate-200 rounded-xl px-8 bg-slate-800">
            <div class="pt-4 md:pt-6text-left divide-y divide-gray-700 space-y-8 md:space-y-12">
                <div>
                    <h1 class={title_cls}>{"What is the Museum of Code?"}</h1>
                    <p class={text_cls}>
                        <span class="font-bold text-cyan-500">{"Museum of Code (MoCo)"}</span>
                        {r#"
                            aims to collect code snippets that are
                            beautiful, elegant, funny, weird, historically significant, or
                            otherwise interesting.
                        "#}
                    </p>
                    <p class={text_cls}>{
                        r#"
                            As you walk/browse through the museum, don't forget to stop for a brief moment
                            and immerse yourself in the context of the artifact. What was the author thinking?
                            How did the language itself help or hinder the author's ability to express their
                            ideas? How could it have been done differently? I hope that you will find
                            the museum to be a place of learning, inspiration and fun.
                        "#
                    }</p>
                </div>

                <div>
                    <h1 class={title_cls}>{"Why MoCo?"}</h1>
                    <p class={text_cls}>{
                        r#"
                            I came up with the idea for MoCo after a trip to Europe. During my visit to
                            numerous museums, I was inspired by the idea of a museum as a place of
                            learning, inspiration, and fun. I wanted to bring that experience to the
                            world of programming. Thus, the Museum of Code was born.
                        "#
                    }</p>
                </div>

                <div>
                    <h1 class={title_cls}>{"Is it free?"}</h1>
                    <p class={text_cls}>
                        <span class="font-semibold text-base text-emerald-300">
                            {"Yes!"}
                        </span>
                    </p>
                    <p class={text_cls}>{
                        r#"
                            The Museum of Code is free and open to the public.
                        "#
                    }</p>
                    <p class={text_cls}>{
                        r#"
                            If you would like to support the museum, you can become a
                            sponsor on GitHub Sponsors. You can also help by submitting
                            your own code snippets (artifacts) to the museum!
                        "#
                    }</p>
                    <div class="flex">
                    <a class="inline-flex mt-4 text-center rounded-full bg-emerald-700 px-3.5 py-2.5 text-xs text-white shadow-sm hover:bg-emerald-500"
                        href="https://github.com/sponsors/YilunAllenChen"
                    >{"Become a Sponsor!"}</a>
                    </div>
                </div>

                <div>
                    <h1 class={title_cls}>{"What is MoCo Built With?"}</h1>
                    <p class={text_cls}>{ r#" The Museum of Code is built with"#}
                        <span class="text-orange-500">{" Rust/Yew "}</span>
                        {"and"}
                        <span class="text-blue-400">{" Tailwind CSS "}</span>
                        {". The code is hosted on GitHub, and the website is hosted on GitHub Pages."}
                    </p>
                    <p class={text_cls}>
                        <span class="text-orange-500">{"Rust"}</span>
                        {r#"
                            was chosen as it is an extremely robust and performant
                            language. If I were to build a museum that stands the test of
                            time, I would want to lay the bricks with a language that can house
                            the sturdiness and quality of such a museum.
                        "#}
                    </p>
                </div>

                <button
                    onclick={nav(Route::Home)}
                    class="rounded-md mt-10 bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white hover:bg-indigo-500">
                    {"Home"}
                </button>
            </div>
            </figure>
        </div>
        }
    }
}
