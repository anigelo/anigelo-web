use yew::prelude::*;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AnimeHeader {
    pub id: String,
    pub title: String,
    pub backdrop: PathBuf,
    pub poster: PathBuf,
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
            <div class="movie_header">
              <img class="locandina" src={format!("file://{}", anime.poster.to_str().unwrap())}/>
              <h1>{&anime.title}</h1>
            </div>
            <div class="movie_desc">
              <p class="text">
                {&anime.description}
              </p>
            </div>
          </div>
          <div class="blur_back bright_back"></div>
        </div>
    }
}