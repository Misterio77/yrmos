[![built with nix](https://img.shields.io/static/v1?logo=nixos&logoColor=white&label=&message=Built%20with%20Nix&color=41439a)](https://builtwithnix.org)
[![hydra status](https://img.shields.io/endpoint?url=https://hydra.m7.rs/job/yrmos/main/x86_64-linux.default/shield)](https://hydra.m7.rs/jobset/yrmos/main#tabs-jobs)

# Sistema Yrmos

MVP de um sistema para organizar caronas dentro de uma organização.

A stack é: Rust (axum, sqlx, maud), PostgreSQL, SCSS.

Build e testes (quando existirem) rodam no meu Hydra: https://hydra.m7.rs/project/yrmos/main.

Temos uma instância rodando em um servidor nosso: https://yrmos.m7.rs.

## TODO

- Modularizar um pouco o código em `routes`
- Implementar busca e filtros
- Ação para remover corridas
- Tela e ação para avaliação de corridas
- Telas e ações relacionadas a perfis

## Como buildar

Você pode buildar mais facilmente usando o [Nix](https://nixos.org): `nix
build`. Você nem precisa clonar o repositório: `nix build
github:misterio77/yrmos`.

Caso você aceite a prompt para o cache, binários pré-buildados pelo CI/CD serão
usados automaticamente, se disponíveis.

Alternativamente, instale o `rustc` e `cargo` da forma que preferir e use
`cargo build`.

## Uso

Basta executar o binário. Ele é 100% autocontido (incluindo CSS, templates,
migrations).

A única dependência de runtime é ter uma base do Postgres.

Use `yrmos --help` para ver todas as opções, elas podem ser configuradas via
CLI ou variáveis de ambiente.

### Produção

Note que, caso executado em modo release, os cookies serão `Secure=true`; isso
significa que não funcionarão sem SSL (exceto se for `localhost`) na maioria
dos navegadores.

Para uso em produção, lembre-se de usar um proxy reverso (especialmente para
ter SSL).

### NixOS

Para usuários de NixOS, provemos um módulo. Importe ele (por exemplo, via
flakes) e use:
```nix
{
  services.yrmos.enable = true;
}
```

Por padrão uma base local `yrmos` será criada, e a autenticação ocorrerá por
Unix Socket. O serviço roda no usuário `yrmos`. Tudo isso é configurável via
options.

Mesmo que você não use NixOS, só o Nix, você pode usar esse módulo numa VM.
Temos uma já pronta em: `nix run .#vm`. O Yrmos estará acessível na porta 8080.
