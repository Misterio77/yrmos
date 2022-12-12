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
struct RegisterForm {
    email: String,
    real_name: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterScreenQuery {
    error: Option<String>,
}

async fn register_screen(
    session: Option<Session>,
    query: Query<RegisterScreenQuery>,
) -> Result<Markup, Redirect> {
    if session.is_some() {
        return Err(Redirect::to("/"));
    }
    let main = html! {
        article {
            header {
                hgroup {
                    h1 {
                        "Registrar"
                    }
                    h2 {
                        "Já tem uma conta? "
                        a href="/login" { "Login" }
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
                    "Nome completo "
                    input name="real_name" autocomplete="name" required;
                }
                label {
                    "Senha "
                    input type="password" name="password" autocomplete="new-password" required;
                }
                button { "Registrar" }
            }
        }
    };
    Ok(layouts::default(main, None))
}

async fn register_action(
    session: Option<Session>,
    cookie_jar: SignedCookieJar,
    State(state): State<AppState>,
    Form(form): Form<RegisterForm>,
) -> Result<(SignedCookieJar, Redirect), Redirect> {
    if session.is_some() {
        return Ok((cookie_jar, Redirect::to("/")));
    }

    Person::register(&state.db_pool, &form.email, &form.password, &form.real_name)
        .await
        .map_err(|e| e.redirect("/register"))?;

    let session = Person::login(&state.db_pool, &form.email, &form.password)
        .await
        .map_err(|e| e.redirect("/register"))?;
    Ok((cookie_jar.add(session.as_cookie()), Redirect::to("/")))
}

pub fn router(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/register", get(register_screen).post(register_action))
        .with_state(state.clone())
}
