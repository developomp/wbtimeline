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
                    category: "development".to_string(),
                    media: None,
                    button: None,
                }} />

                <Entry data={Data {
                    date: "NOW".to_string(),
                    title: "You can be part of WB history".to_string(),
                    description: "Join the War Brokers discord community and be a part of War Brokers' glorious history!".to_string(),
                    category: "community".to_string(),
                    media: None,
                    button: Some(["https://discord.gg/warbrokers".to_string(), "War Brokers discord server".to_string()]),
                }} />
            </div>
        </div>
    }
}
