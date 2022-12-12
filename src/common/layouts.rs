use maud::{html, Markup, DOCTYPE};

pub fn default(content: Markup) -> Markup {
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
                (navbar())
                main .container { (content) }
                (footer())
            }
        }
    }
}

pub fn navbar() -> Markup {
    html! {
        #navbar {
            nav .container-fluid {
              menu {}
              a .logo href="/" { "Yrmos" }
              menu {}
            }
            nav .container-fluid.search {
              form action="/rides" {
                input type="search" placeholder="Destino, origem...";
                a { "Filtros" }
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
