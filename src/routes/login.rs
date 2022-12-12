use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
    Form, Router,
};
use axum_extra::extract::SignedCookieJar;
use maud::{html, Markup};
use serde::Deserialize;

use crate::{
    layouts,
    schema::{Person, Session},
    state::AppState,
};

#[derive(Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginScreenQuery {
    error: Option<String>,
}

async fn login_screen(
    session: Option<Session>,
    query: Query<LoginScreenQuery>,
) -> Result<Markup, Redirect> {
    if session.is_some() {
        return Err(Redirect::to("/"));
    }
    let main = html! {
        article {
            header {
                hgroup {
                    h1 {
                        "Login"
                    }
                    h2 {
                        "Não tem uma conta? "
                        a href="/register" { "Registrar" }
                    }
                }
                @if let Some(flash_message) = &query.error {
                    (layouts::flash(flash_message, "error"))
                }
            }
            form method="post" {
                label {
                    "Email "
                    input name="email" type="email" autocomplete="email" required autofocus;
                }
                label {
                    "Senha "
                    input type="password" name="password" autocomplete="current-password" required;
                }
                button { "Login" }
            }
        }
    };
    Ok(layouts::default(main, None))
}

async fn login_action(
    session: Option<Session>,
    cookie_jar: SignedCookieJar,
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<(SignedCookieJar, Redirect), Redirect> {
    if session.is_some() {
        return Ok((cookie_jar, Redirect::to("/")));
    }

    let session = Person::login(&state.db_pool, &form.email, &form.password)
        .await
        .map_err(|e| e.redirect("/login"))?;
    Ok((cookie_jar.add(session.as_cookie()), Redirect::to("/")))
}

pub fn router(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/login", get(login_screen).post(login_action))
        .with_state(state.clone())
}
