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
                    date: "Nov 13, 2017".to_string(),
                    title: "Release of open beta for browsers".to_string(),
                    description: "The grand beginning of the legacy.".to_string(),
                    category: "development".to_string(),
                    media: None,
                    button: None,
                }} />

                <Entry data={Data {
                    date: "Some date".to_string(),
                    title: "Some title".to_string(),
                    description: "Some long detailed well-written yet concise description.".to_string(),
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
