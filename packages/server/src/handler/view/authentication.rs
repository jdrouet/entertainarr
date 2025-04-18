use axum::{
    extract::FromRequestParts,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::CookieJar;

#[derive(Clone, Debug)]
pub struct Authentication(pub u64);

impl<S> FromRequestParts<S> for Authentication {
    type Rejection = Redirect;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let jar = CookieJar::from_headers(&parts.headers);

        async move {
            let user_id = jar
                .get("user_id")
                .ok_or_else(|| Redirect::temporary("/login"))?;
            let user_id = user_id
                .value()
                .parse::<u64>()
                .map_err(|_| Redirect::temporary("/login"))?;

            Ok(Authentication(user_id))
        }
    }
}
