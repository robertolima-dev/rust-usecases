# 🚀 Rust API - Actix Web + PostgreSQL + MongoDB + ElasticSearch

Este projeto é uma API RESTful robusta desenvolvida em [Rust 🦀](https://www.rust-lang.org/), utilizando **Actix Web** como framework web, **PostgreSQL** como banco de dados principal, **MongoDB** como suporte para sistema de logs estruturados e **Elasticsearch** para busca full-text. A aplicação está organizada com foco em modularidade, escalabilidade, segurança e boas práticas.


## ✅ Funcionalidades Implementadas

* ✅ Criar usuário (`POST /api/v1/users/`)
* ✅ Login e geração de JWT (`POST /api/v1/login/`)
* ✅ Obter perfil autenticado (`GET /api/v1/me/`)
* ✅ Middleware JWT (`Authorization: Token <JWT>`)
* ✅ Extração de `user_id` via trait: `req.user_id()?`
* ✅ Padronização de erros com `AppError`
* ✅ Sistema de logs com MongoDB
* ✅ Confirmação de e-mail e redefinição de senha com códigos temporários
* ✅ Repositórios para encapsular queries SQL
* ✅ Migrations com SQLx
* ✅ Estrutura modular e escalável
* ✅ Hot reload com `cargo watch`
* ✅ Sistema de tokens para ações temporárias (UserToken)
* ✅ Módulo de cursos com busca full-text no Elasticsearch


## 🚀 Tecnologias

* [Rust](https://www.rust-lang.org/)
* [Actix Web](https://actix.rs/)
* [SQLx](https://github.com/launchbadge/sqlx)
* [PostgreSQL](https://www.postgresql.org/)
* [MongoDB](https://www.mongodb.com/)
* [Elasticsearch](https://www.elastic.co/)
* [JWT](https://jwt.io/)
* [Tera Templates](https://tera.netlify.app/) – para e-mails HTML
* [Tracing](https://github.com/tokio-rs/tracing)


## 📋 Pré-requisitos

* Rust (versão estável)
* PostgreSQL
* MongoDB
* Docker (opcional)


## 🔧 Instalação

1. Clone o repositório:

```bash
git clone https://github.com/seu-usuario/rust-usecases.git
cd rust-usecases
```

2. Configure o banco de dados:

```bash
createdb rust_usecases
```

3. Copie e edite o `.env`:

```bash
cp .env.example .env
```

4. Execute as migrations:

```bash
sqlx database create
sqlx migrate run
```

5. Rode o projeto:

```bash
./start_server.sh
```


## 🏗️ Estrutura do Projeto

```
src/
├── config/          # Configurações da aplicação
├── db/              # Inicialização do banco de dados
├── errors/          # AppError e mapeamento de erros
├── extensions/      # Traits auxiliares como RequestUserExt
├── middleware/      # JWT Middleware
├── models/          # Structs principais (User, Profile, etc)
├── repositories/    # Acesso ao banco de dados (CRUD SQL)
├── routes/          # Rotas organizadas por módulo
├── services/        # Lógica de negócio da aplicação
├── logs/            # Integração com MongoDB + macros de log
├── utils/           # Funções auxiliares (JWT, validação, etc)
└── main.rs          # Entry point
```


## 🌟 Funcionalidades

### 🔐 Autenticação

* Login via e-mail/senha
* JWT Token com expiração configurável
* Middleware que injeta `Claims` e `user_id` na request

### 👤 Gerenciamento de Usuários

* Registro com hash de senha
* Atualização de nome/sobrenome apenas pelo dono do perfil
* Soft delete (`dt_deleted`)
* Recuperação e confirmação de e-mail com hash expirável

### 🔑 Sistema de Tokens (UserToken)

* Tokens temporários para ações específicas
* Tipos de token: `confirm_email`, `change_password`
* Expiração automática após 180 minutos
* Controle de consumo único (consumed)
* Validação de tipo, expiração e consumo
* Reutilização segura de tokens já consumidos

### 🔄 Fluxo de Redefinição de Senha

1. Usuário solicita redefinição (`POST /forgot-password/`)
   - Envia email
   - Sistema gera token temporário
   - Token enviado por email (a ser implementado)

2. Usuário redefine senha (`POST /change-password/`)
   - Envia token e nova senha
   - Sistema valida token
   - Atualiza senha e marca token como usado
   - Retorna sucesso mesmo se token já foi usado

### 📬 Templates de E-mail

* Templates renderizados com [Tera](https://tera.netlify.app/)
* `reset_password.html`, `confirm_email.html`, etc.

### 🧠 Validações

* Customizadas com [validator](https://crates.io/crates/validator)
* E-mails, senhas, documentos, telefones, etc.

### 📜 Sistema de Logs

* Armazenamento no MongoDB
* Macro `log_fail!` para falhas com contexto (user\_id, módulo, mensagem)
* Logs consultáveis via endpoint: `GET /api/v1/logs/?level=Error&limit=10`

### 🧰 Utilitários

* `AppError` unificado com `ResponseError`
* Macros de log (`log_fail!`, `log_info!`)
* Trait para acessar dados da request (`RequestUserExt`)

### 📚 Módulo de Cursos

* Gerenciamento completo de cursos
* Sincronização bidirecional com Elasticsearch
* Busca full-text com paginação
* Endpoints:
  * `POST /api/v1/courses/` - Criar curso
  * `PUT /api/v1/courses/{id}/` - Atualizar curso
  * `GET /api/v1/courses/` - Buscar cursos

#### 🔍 Integração com Elasticsearch

* Índice dinâmico com prefixo configurável
* Sincronização automática ao criar/atualizar
* Busca full-text em múltiplos campos
* Paginação de resultados
* Configuração via variáveis de ambiente:
  * `ELASTICSEARCH_URL`
  * `ELASTICSEARCH_INDEX_PREFIX`


## 🔍 Exemplo de Uso de Logs

```rust
log_fail!(
    err,
    LogLevel::Error,
    "Erro ao buscar usuário",
    "user_service",
    Some(user_id),
    mongo_db
);
```


## 🧪 Migrations com SQLx

```bash
sqlx migrate add nome_migration
sqlx migrate run
```


## 📡 Exemplos de Endpoints

| Método | Rota                | Descrição                          | Auth |
| ------ | ------------------- | ---------------------------------- | ---- |
| POST   | `/api/v1/users/`    | Criação de usuário                 | ❌    |
| POST   | `/api/v1/login/`    | Login e geração de token           | ❌    |
| GET    | `/api/v1/me/`       | Obter dados do usuário logado      | ✅    |
| PUT    | `/api/v1/users/`    | Atualizar nome/sobrenome           | ✅    |
| DELETE | `/api/v1/users/`    | Soft delete no próprio usuário     | ✅    |
| POST   | `/api/v1/profiles/` | Atualizar perfil do usuário logado | ✅    |
| GET    | `/api/v1/logs/`     | Consultar logs do MongoDB          | ✅    |
| GET    | `/api/v1/confirm-email/{code}/` | Confirmar email do usuário | ❌    |
| POST   | `/api/v1/forgot-password/` | Solicitar redefinição de senha | ❌    |
| POST   | `/api/v1/change-password/` | Redefinir senha com token | ❌    |
| POST   | `/api/v1/courses/`  | Criar novo curso                   | ✅    |
| PUT    | `/api/v1/courses/{id}/` | Atualizar curso existente    | ✅    |
| GET    | `/api/v1/courses/`  | Buscar cursos (full-text)          | ✅    |
| DELETE    | `/api/v1/courses/{id}`  | Soft delete de cursos       | ✅    |


## ✉️ Templates de E-mail (Tera)

```rust
let mut ctx = tera::Context::new();
ctx.insert("name", &user.first_name);
ctx.insert("link", &reset_link);

let body = tera.render("emails/reset_password.html", &ctx)?;
```


## ✍️ Autor

**Roberto Lima**
[🔗 GitHub](https://github.com/robertolima-dev) — [🌐 Portfólio](https://robertolima-developer.vercel.app)
📧 [robertolima.izphera@gmail.com](mailto:robertolima.izphera@gmail.com)


## 📜 Licença

MIT © 2025 — Livre para uso, estudo e modificação.
