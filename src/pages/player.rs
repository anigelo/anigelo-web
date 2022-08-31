use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
    pub id: String,
    pub season: u8,
    pub episode: u32
}

#[function_component(Player)]
pub fn episode_list(PlayerProps { id, season, episode }: &PlayerProps) -> Html {
    html! { <h1>{ format!("Id: {}, Season: {}, Episode: {}", id, season, episode) }</h1> }
}