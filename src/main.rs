use yew::prelude::*;

struct Index;
impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="bg-black h-full">
            <header class="fixed inset-x-0 top-0 z-50">
                <nav class="flex items-center justify-between p-6 lg:px-8" aria-label="Global">
                    <div class="flex lg:flex-1">
                        // logo
                    </div>
                    <div class="flex lg:hidden">
                    </div>
                    <div class="hidden lg:flex lg:gap-x-12 lg:justify-around">
                        <a href="#" class="text-sm font-semibold leading-6 text-gray-100">{"WIP"}</a>
                        <a href="#" class="text-sm font-semibold leading-6 text-gray-100">{"WIP"}</a>
                        <a href="#" class="text-sm font-semibold leading-6 text-gray-100">{"WIP"}</a>
                    </div>
                    <div class="hidden lg:flex lg:flex-1 lg:justify-end">
                    </div>
                </nav>
            </header>
            <div class="relative isolate px-6 pt-14 lg:px-8">
                <div class="absolute inset-x-0 -top-40 -z-10 transform-gpu overflow-hidden blur-3xl sm:-top-80" aria-hidden="true">
                    <div class="relative left-[calc(50%-11rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 rotate-[30deg] bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%-30rem)] sm:w-[72.1875rem]"
                        style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)">
                    </div>
                </div>
                <div class="mx-auto max-w-2xl py-32 sm:py-48 lg:py-56">
                    <div class="text-center">
                        <h1 class="text-4xl font-bold tracking-tight text-gray-100 sm:text-6xl">{"A Museum of Code"}</h1>
                        <p class="mt-6 text-lg leading-8 text-gray-300">{"Code that ought to be put in a museum."}</p>
                        <div class="mt-10 flex items-center justify-center gap-x-6 mb-8">
                            <a href="#"
                                class="rounded-md disabled bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                                {"Take A Tour"}
                            </a>
                            <a href="#" class="text-sm font-semibold leading-6 text-gray-100">{"(Gallery under construction)"}</a>
                        </div>
                    </div>
                    <div class="bg-gray-800 lg:pl-20 text-gray-300 p-4  rounded-md justify-left items-left">
                        <pre>
                            <code class="text-sm font-mono text-left">{r#"
-- | Quicksort in Haskell
qs :: (Ord a) => [a] -> [a]
qs [] = []
qs (x:xs) = qs [y | y <- xs, y <= x]
            ++ [x] ++
            qs [y | y <- xs, y > x]

      "#}
                            </code>
                        </pre>
                    </div>
                </div>
                <div class="absolute inset-x-0 top-[calc(100%-13rem)] -z-10 transform-gpu overflow-hidden blur-3xl sm:top-[calc(100%-30rem)]" aria-hidden="true">
                    <div class="relative left-[calc(50%+3rem)] aspect-[1155/678] w-[36.125rem] -translate-x-1/2 bg-gradient-to-tr from-[#ff80b5] to-[#9089fc] opacity-30 sm:left-[calc(50%+36rem)] sm:w-[72.1875rem]"
                        style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)">
                    </div>
                </div>
            </div>
        </div>

          }
    }
}

fn main() {
    yew::start_app::<Index>();
}
