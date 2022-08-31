use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;
use crate::components::anime_card::AnimeCard;

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct AnimeHeader {
    pub id: String,
    pub title: String,
    pub backdrop: String,
    pub poster: String,
    pub description: String
}

#[function_component(AnimeList)]
pub fn anime_list() -> Html {
    let animes = use_state(|| vec![]);
    {
        let animes = animes.clone();
        use_effect_with_deps(move |_| {
            let animes = animes.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_animes = Request::get("/api/media")
                    .send().await.unwrap()
                    .json::<Vec<AnimeHeader>>().await.unwrap();
                animes.set(fetched_animes);
            });
            || ()
        }, ());
    }


    let list = animes.iter()
        .map(view_list_item)
        .collect::<Html>();

    html! {
        <section>
            {list}
        </section>
    }
}

fn view_list_item(anime: &AnimeHeader) -> Html {
    html! {
        <AnimeCard
            poster={anime.poster.clone()}
            title={anime.title.clone()}
            description={anime.description.clone()}
            backdrop={anime.backdrop.clone()}
        />
    }
}