use maud::{html, Markup, DOCTYPE};

use crate::{
    icons::{ACCOUNT_CIRCLE, FILTER_LIST, LOGIN, YRMOS_LOGO, LOGOUT},
    schema::Session,
};

pub fn default(content: Markup, session: Option<&Session>) -> Markup {
    root(content, session, true)
}

pub fn root(content: Markup, session: Option<&Session>, show_session: bool) -> Markup {
    html! {
        (DOCTYPE)
        html lang="pt-br" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "Yrmos" }
                link rel="stylesheet" href="/assets/style.css";
            }
            body {
                (navbar(session, show_session))
                main .container { (content) }
                (footer())
            }
        }
    }
}

pub fn navbar(session: Option<&Session>, show_session: bool) -> Markup {
    let logo = html!{a .logo href="/" { (YRMOS_LOGO) "Yrmos" }};
    html! {
        #navbar {
            nav .container-fluid.main {
                @if show_session {
                    @if let Some(sess) = session {
                        a href={"/profiles/" (sess.creator)} {
                            "Perfil " (ACCOUNT_CIRCLE)
                        }
                        (logo)
                        a href="/logout" {
                            "Logout " (LOGOUT)
                        }
                    } @else {
                        span {}
                        (logo)
                        a href="/login" {
                            "Login " (LOGIN)
                        }
                    }
                } @else {
                    (logo)
                }
            }
            nav .container-fluid.search {
                form action="/rides" {
                    input type="search" placeholder="Destino, origem...";
                    a { "Filtros" (FILTER_LIST) }
                }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        footer .container {
            "Desenvolvido para a disciplina de empreendedorismo no "
            a href="https://icmc.usp.br" { "ICMC-USP" }
            " (" a href="https://github.com/misterio77/yrmos" { "Código fonte" } ")"
        }
    }
}

pub fn flash(message: &str, severity: &str) -> Markup {
    html! {
        #flash .(severity) {
            sup onclick="document.getElementById('flash').remove()" {
                (message)
            }
        }
    }
}
