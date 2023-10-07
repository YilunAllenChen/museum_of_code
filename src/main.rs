use yew::prelude::*;
mod highlighter;
use highlighter::Highlighter;

#[function_component]
fn App() -> Html {
    let haskell = r#"
-- QuickSort in Haskell
qs :: (Ord a) => [a] -> [a]
qs [] = []
qs (x:xs) = qs bot ++ [x] ++ qs top
  where
    bot = [y | y <- xs, y <= x]
    top = [y | y <- xs, y > x]
    "#;

    let formatted_haskell = Highlighter::new().highlight(haskell.to_string());

    let particles = html! {
        <div>
            <div>
                { for (0..200).map(|_| {
                    let top = format!("{}vh", rand::random::<f32>() * 200.0 - 100.0);
                    let left = format!("{}vw", rand::random::<f32>() * 200.0 - 100.0);
                    html! {
                        <div class="particle" style={format!("top: {}; left: {};", top, left)}>
                        </div>
                    }
                })}
            </div>
        </div>
    };

    html! {
        <div class="bg-black h-full animation-container">
            <div class="relative isolate px-6 pt-14 lg:px-8">
                <div class="mx-auto max-w-2xl py-32 sm:py-48 lg:py-56 bg-opacity-50">
                    <div class="text-center">
                        <h1 class="text-4xl font-bold tracking-tight text-gray-100 sm:text-6xl">{"A Museum of Code"}</h1>
                        <p class="mt-6 text-lg leading-8 text-gray-300">{"Code that ought to be put in a museum."}</p>
                        <div class="mt-10 flex items-center justify-center gap-x-6 mb-20">
                            <a href="#"
                                class="rounded-md disabled bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                                {"Take A Tour"}
                            </a>
                            <a href="#" class="text-sm font-semibold leading-6 text-gray-100">{"(Gallery under construction)"}</a>
                        </div>
                    </div>
                    <div class="bg-gray-800 lg:pl-20 text-gray-300 p-4  rounded-md justify-left items-left">
                        <pre>
                            {formatted_haskell}
                        </pre>
                    </div>
                </div>
                {particles}
            </div>
        </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
