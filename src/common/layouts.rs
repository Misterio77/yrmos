use maud::{html, Markup, DOCTYPE};

use crate::{
    icons::{FILTER_LIST, LOGIN, LOGOUT, YRMOS_LOGO},
    schema::Session,
};

pub fn default(content: Markup, session: Option<&Session>) -> Markup {
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
                (navbar(session))
                main .container { (content) }
                (footer())
            }
        }
    }
}

pub fn navbar(session: Option<&Session>) -> Markup {
    html! {
        #navbar {
            nav .container-fluid.main {
              menu {}
              a .logo href="/" { (YRMOS_LOGO) "Yrmos" }
              @if let Some(sess) = session {
                  a href={"/profiles/" (sess.creator)} {
                      "Perfil " (LOGOUT)
                  }
              } @else {
                  a href="/login" {
                      "Login " (LOGIN)
                  }
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
