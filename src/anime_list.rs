use yew::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AnimeHeader {
    pub id: String,
    pub title: String,
    pub backdrop: String,
    pub poster: String,
    pub description: String
}

#[derive(Properties, PartialEq)]
pub struct AnimeListProps {
    pub animes: Vec<AnimeHeader>,
}

#[function_component(AnimeList)]
pub fn anime_list(AnimeListProps { animes }: &AnimeListProps) -> Html {
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
        <div class="anime_card">
          <div class="info_section">
            <div class="anime_header">
              <img class="locandina" src={format!("http://localhost:8088/static/{}", anime.poster)}/>
              <h1>{&anime.title}</h1>
            </div>
            <div class="anime_desc">
              <p class="text">
                {&anime.description}
              </p>
            </div>
          </div>
          <div class="blur_back" style={format!("background: url('http://localhost:8088/static/{}');", anime.backdrop)}></div>
        </div>
    }
}