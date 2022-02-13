use reqwasm::http::Request;
use yew::{function_component, html, use_effect_with_deps, use_state};

use crate::components::{Entry, Header};
use crate::data::Data;

#[function_component(App)]
pub fn app() -> Html {
    let data = use_state(|| Vec::new());

    {
        let data = data.clone();

        use_effect_with_deps(
            move |_| {
                let data = data.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let res = Request::get("/data.json").send().await.unwrap();

                    if res.status() != 200 {
                        log::error!("Failed to fetch data");
                    }

                    data.set(
                        serde_json::from_str::<Vec<Data>>(&res.text().await.unwrap()).unwrap(),
                    );
                });

                || ()
            },
            (),
        );
    }

    let entries = data.iter().map(|data| {
        html! {
           <Entry data={Data {
               date: data.date.to_string(),
               title: data.title.to_string(),
               description: data.description.to_string(),
               category: data.category.to_string(),
               subcategory: data.subcategory.to_string(),
               media: None,
               button: None,
           }} />
        }
    });

    html! {
        <div class="container">
            <Header />

            <div class="timeline">
                { for entries }

                <Entry data={Data {
                    date: "NOW".to_string(),
                    title: "You can be part of the history".to_string(),
                    description: "Join the War Brokers discord community today and be a part of the history!".to_string(),
                    category: "community".to_string(),
                    subcategory: "".to_string(),
                    media: None,
                    button: Some(["https://discord.gg/warbrokers".to_string(), "War Brokers discord server".to_string()]),
                }} />
            </div>
        </div>
    }
}
