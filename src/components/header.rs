use yew::{function_component, html};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <h1>
                { "War Brokers Timeline" }
            </h1>

            { "Made by " }
            <a href="https://github.com/developomp" target="_blank">
                { "[LP] POMP (developomp)" }
            </a>
        </header>
    }
}
