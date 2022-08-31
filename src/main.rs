mod pages;
mod components;

use dotenv::dotenv;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::anime_list::{AnimeList};
use crate::pages::season_list::SeasonList;
use crate::pages::episode_list::EpisodeList;
use crate::pages::player::Player;

fn main() {
    dotenv().ok();
    yew::start_app::<App>();
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/:id")]
    Seasons { id: String },
    #[at("/:id/s/:season")]
    Episodes { id: String, season: u8 },
    #[at("/:id/s/:season/ep/:episode")]
    Playback { id: String, season: u8, episode: u32 }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => html! { <AnimeList /> },
        Route::Seasons { id} => html! { <SeasonList id={id} /> },
        Route::Episodes { id, season } => html! { <EpisodeList id={id} season={season} />},
        Route::Playback { id, season, episode} => html! { <Player id={id} season={season} episode={episode} /> }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
        </BrowserRouter>
    }
}