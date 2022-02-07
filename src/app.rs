use yew::{function_component, html};

use crate::components::{Entry, Header};
use crate::data::Data;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <Header />

            <div class="timeline">
                <Entry data={Data {
                    date: "Mar 18, 2020".to_string(),
                    title: "Here be item 1".to_string(),
                    description: "Long text. Long text. Long text. Long text. Long text.".to_string(),
                    media: None,
                }} />

                <Entry data={Data {
                    date: "Mar 26, 2020".to_string(),
                    title: "And here be item 2".to_string(),
                    description: "Long text. Long text. Long text. Long text. Long text.".to_string(),
                    media: None,
                }} />
            </div>
        </div>
    }
}
