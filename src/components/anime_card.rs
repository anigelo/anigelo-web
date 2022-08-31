use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AnimeCardProps {
    pub poster: String,
    pub title: String,
    pub description: String,
    pub backdrop: String
}

#[function_component(AnimeCard)]
pub fn anime_card(AnimeCardProps {poster, title, description, backdrop}: &AnimeCardProps) -> Html {
    html! {
        <div class="anime_card">
          <div class="info_section">
            <div class="anime_header">
              <img class="locandina" src={format!("/api/static/{}", poster)}/>
              <h1>{&title}</h1>
            </div>
            <div class="anime_desc">
              <p class="text">
                {&description}
              </p>
            </div>
          </div>
          <div class="blur_back" style={format!("background: url('/api/static/{}');", backdrop)}></div>
        </div>
    }
}