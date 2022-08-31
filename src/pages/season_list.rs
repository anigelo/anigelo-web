use yew::prelude::*;
use serde::Deserialize;
use reqwasm::http::Request;

#[derive(Properties, PartialEq)]
pub struct SeasonListProps {
    pub id: String
}

#[derive(Deserialize, PartialEq, Debug)]
struct AnimeSeason {
    number: u8,
    poster: String,
    episodes: u32
}

#[function_component(SeasonList)]
pub fn season_list(SeasonListProps { id }: &SeasonListProps) -> Html {
    let request_url = format!("/api/media/{}/seasons", id);
    let seasons = use_state(|| vec![]);
    {
        let seasons = seasons.clone();
        use_effect_with_deps(move |_| {
            let seasons = seasons.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_seasons = Request::get(&request_url)
                    .send().await.unwrap()
                    .json::<Vec<AnimeSeason>>().await.unwrap();
                seasons.set(fetched_seasons);
            });
            || ()
        }, ());
    }


    let list = seasons.iter()
        .map(view_list_item)
        .collect::<Html>();

    html! {
        <section>
            {list}
        </section>
    }
}

fn view_list_item(season: &AnimeSeason) -> Html {
    html! {
        <div class="anime_card">
          <div class="info_section">
            <div class="anime_header">
              <img class="locandina" src={format!("/api/static/{}", season.poster)}/>
              <h1>{format!("Season {}", season.number)}</h1>
            </div>
            <div class="anime_desc">
              <p class="text">
                {format!("{} episodes", season.episodes)}
              </p>
            </div>
          </div>
        </div>
    }
}