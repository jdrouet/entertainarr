use yew::prelude::*;
use yew_router::prelude::*;

mod component;
mod hook;
mod view;

use hook::user::use_me;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum AuthenticationState {
    #[default]
    Loading,
    Authenticated,
    Anonymous,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct AuthenticationContext {
    pub state: AuthenticationState,
    pub loading: bool,
    pub refresh: Callback<()>,
}

#[hook]
fn use_authentication_context() -> AuthenticationContext {
    let user = use_me();

    let state = match user.data {
        Some(_) => AuthenticationState::Authenticated,
        None if user.loading => AuthenticationState::Loading,
        None => AuthenticationState::Anonymous,
    };

    AuthenticationContext {
        state,
        loading: user.loading,
        refresh: Callback::from(move |_| {
            user.run();
        }),
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/tvshows")]
    TvshowIndex,
    #[at("/tvshows/search")]
    TvshowSearch,
    #[at("/tvshows/:tvshow_id")]
    TvshowView { tvshow_id: u64 },
    #[at("/tvshows/:tvshow_id/seasons/:season_number")]
    TvshowSeasonView { tvshow_id: u64, season_number: u64 },
    #[at("/tvshows/:tvshow_id/seasons/:season_number/episodes/:episode_number")]
    TvshowEpisodeView {
        tvshow_id: u64,
        season_number: u64,
        episode_number: u64,
    },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch_anonymous(routes: Route) -> Html {
    match routes {
        Route::Login => {
            use view::login::Login;
            html! { <Login redirect={None} /> }
        }
        other => {
            use view::login::Login;
            html! { <Login redirect={Some(other)} /> }
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            use view::home::Home;
            html! { <Home /> }
        }
        Route::Login => {
            html! { <Redirect<Route> to={Route::Home} /> }
        }
        Route::TvshowIndex => {
            use view::tvshow_index_view::TVShowIndex;
            html! { <TVShowIndex /> }
        }
        Route::TvshowSearch => {
            use view::tvshow_search::TVShowSearch;
            html! { <TVShowSearch /> }
        }
        Route::TvshowView { tvshow_id } => {
            use view::tvshow_view::TVShowView;
            html! { <TVShowView {tvshow_id} /> }
        }
        Route::TvshowSeasonView {
            tvshow_id,
            season_number,
        } => {
            use view::tvshow_season_view::TVShowSeasonView;
            html! { <TVShowSeasonView {tvshow_id} {season_number} /> }
        }
        Route::TvshowEpisodeView {
            tvshow_id,
            season_number,
            episode_number,
        } => {
            use view::tvshow_episode_view::TVShowEpisodeView;
            html! { <TVShowEpisodeView {tvshow_id} {season_number} {episode_number} /> }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
pub fn App() -> Html {
    let auth_ctx = use_authentication_context();

    if matches!(auth_ctx.state, AuthenticationState::Loading) {
        return html! {<div />};
    }

    let is_anonymous = matches!(auth_ctx.state, AuthenticationState::Anonymous);

    html! {
        <ContextProvider<AuthenticationContext> context={auth_ctx}>
            <BrowserRouter>
                if is_anonymous {
                    <Switch<Route> render={switch_anonymous} />
                } else {
                    <Switch<Route> render={switch} />
                }
            </BrowserRouter>
        </ContextProvider<AuthenticationContext>>
    }
}
