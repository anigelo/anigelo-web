use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;

#[derive(Properties, PartialEq)]
pub struct EpisodeListProps {
    pub id: String,
    pub season: u8
}

#[derive(Deserialize, PartialEq, Debug)]
struct AnimeEpisode {
    number: u32,
    title: String,
    video: String,
    description: String,
    thumbnail: String
}

#[derive(Deserialize, PartialEq, Debug)]
struct AnimeSeason {
    number: u8,
    poster: String,
    episodes: Vec<AnimeEpisode>
}

#[function_component(EpisodeList)]
pub fn episode_list(EpisodeListProps { id, season }: &EpisodeListProps) -> Html {
    let request_url = format!("/api/media/{}/seasons/{}", id, season);
    let episodes = use_state(|| vec![]);
    {
        let episodes = episodes.clone();
        use_effect_with_deps(move |_| {
            let episodes = episodes.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_season = Request::get(&request_url)
                    .send().await.unwrap()
                    .json::<AnimeSeason>().await.unwrap();
                episodes.set(fetched_season.episodes);
            });
            || ()
        }, ());
    }


    let list = episodes.iter()
        .map(view_list_item)
        .collect::<Html>();

    html! {
        <section>
            {list}
        </section>
    }
}

fn view_list_item(episode: &AnimeEpisode) -> Html {
    html! {
        <div class="anime_card">
          <div class="info_section">
            <div class="anime_header">
              <img class="locandina" src={format!("/api/static/{}", episode.thumbnail)}/>
              <h1>{&episode.title}</h1>
            </div>
            <div class="anime_desc">
              <p class="text">
                {&episode.description}
              </p>
            </div>
          </div>
        </div>
    }
}