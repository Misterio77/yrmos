use axum::{extract::State, response::Redirect, routing::get, Form, Router};
use axum_extra::extract::SignedCookieJar;
use maud::{html, Markup};
use serde::Deserialize;

use crate::{layouts, schema::Session, state::AppState, AppError};

async fn logout_screen(session: Session) -> Markup {
    let main = html! {
        article {
            header {
                h1 {
                    "Logout"
                }
            }
            form method="post" {
                fieldset {
                    label {
                        input role="switch" type="checkbox" name="all" value="true";
                        "Sair de todas as sessões"
                    }
                }
                button { "Sair" }
            }
        }
    };
    layouts::default(main, Some(&session))
}

#[derive(Deserialize)]
struct LogoutForm {
    #[serde(default)]
    all: bool,
}

async fn logout_action(
    session: Session,
    cookie_jar: SignedCookieJar,
    State(state): State<AppState>,
    Form(form): Form<LogoutForm>,
) -> Result<(SignedCookieJar, Redirect), AppError> {
    if form.all {
        session.revoke_all(&state.db_pool).await?;
    } else {
        session.revoke_self(&state.db_pool).await?;
    }
    Ok((cookie_jar.remove(session.as_cookie()), Redirect::to("/")))
}

pub fn router(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/logout", get(logout_screen).post(logout_action))
        .with_state(state.clone())
}
