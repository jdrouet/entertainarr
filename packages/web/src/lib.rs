use yew::prelude::*;
use yew_router::prelude::*;

mod component;
mod view;

#[derive(Clone, Debug, Default, PartialEq)]
struct Authentication {
    pub user_id: Option<u64>,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/tvshows/search")]
    TvshowSearch,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            use view::home::Home;
            html! { <Home /> }
        }
        Route::Login => {
            use view::login::Login;
            html! { <Login /> }
        }
        Route::TvshowSearch => {
            use view::tvshow_search::TVShowSearch;
            html! { <TVShowSearch /> }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
pub fn App() -> Html {
    let ctx = use_state(|| Authentication::default());

    html! {
        <ContextProvider<Authentication> context={(*ctx).clone()}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<Authentication>>
    }
}
