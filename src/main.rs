mod anime_list;

use dotenv::dotenv;
use reqwasm::http::Request;
use yew::prelude::*;
use crate::anime_list::{AnimeList,AnimeHeader};

fn main() {
    dotenv().ok();

    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    let animes = use_state(|| vec![]);
    {
        let animes = animes.clone();
        use_effect_with_deps(move |_| {
            let animes = animes.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_animes = Request::get("http://localhost:8088/media")
                    .send().await.unwrap()
                    .json::<Vec<AnimeHeader>>().await.unwrap();
                animes.set(fetched_animes);
            });
            || ()
        }, ());
    }

    html! {
            <main>
                <AnimeList animes={(*animes).clone()} />
            </main>
        }
}