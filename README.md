# 🚀 Rust API - Users Service com Actix Web + PostgreSQL

Este projeto é uma API RESTful robusta desenvolvida em [Rust 🦀](https://www.rust-lang.org/), utilizando **Actix Web** como framework web e **PostgreSQL** como banco de dados. A aplicação está estruturada de forma modular com foco em boas práticas, segurança e escalabilidade.

---

## ✅ Funcionalidades Implementadas

- ✅ Criar usuário (`POST /api/v1/users/`)
- ✅ Login e geração de JWT (`POST /api/v1/login/`)
- ✅ Obter perfil autenticado (`GET /api/v1/me/`)
- ✅ Módulo de autenticação com middleware
- ✅ Middleware JWT (`Authorization: Token <JWT>`)
- ✅ Extração de `user_id` via trait: `req.user_id()?`
- ✅ Padronização de erros com `AppError`
- ✅ Armazenamento de dados com PostgreSQL
- ✅ Migrations com SQLx
- ✅ Hot Reload com `cargo watch`
- ✅ Estrutura modular (routes, services, models, errors, middleware)

---

## 📂 Estrutura de Pastas

```

src/
├── db/                         # Conexão com o banco
├── errors/                     # AppError e tratamentos customizados
├── extensions/                 # Traits como RequestUserExt
├── middleware/                 # Middleware de autenticação JWT
├── models/                     # Structs de User, Profile, etc
├── routes/                     # Rotas agrupadas por versão
├── services/                   # Regras de negócio (login, users, auth)
├── main.rs                     # Entrada principal

````

---

## 📡 Endpoints disponíveis

| Método | Rota              | Descrição                              | Auth |
|--------|-------------------|----------------------------------------|------|
| POST   | `/api/v1/users/`  | Criação de usuário                     | ❌    |
| POST   | `/api/v1/login/`  | Login e retorno do JWT                 | ❌    |
| GET    | `/api/v1/me/`     | Obter dados do usuário autenticado     | ✅    |

### 🔐 Headers

Para rotas protegidas:
```http
Authorization: Token <JWT>
````

### 📥 Exemplo de JSON para criação de usuário

```json
{
  "email": "roberto@email.com",
  "first_name": "Roberto",
  "last_name": "Lima",
  "password": "senha123"
}
```

---

## 🔧 Como rodar localmente

### 1. Clone o projeto

```bash
git clone https://github.com/seu-usuario/seu-projeto.git
cd seu-projeto
```

### 2. Crie o `.env`

```env
DATABASE_URL=postgres://usuario:senha@localhost:5432/seu_banco
JWT_SECRET=sua_chave_secreta
JWT_EXPIRES_IN=86400
```

### 3. Execute as migrations e rode o projeto

```bash
./start_server.sh
```

---

## 🛠️ Tecnologias utilizadas

* [Rust](https://www.rust-lang.org/)
* [Actix Web](https://actix.rs/)
* [PostgreSQL](https://www.postgresql.org/)
* [SQLx](https://docs.rs/sqlx/)
* [JWT (jsonwebtoken)](https://docs.rs/jsonwebtoken/)
* [Serde](https://serde.rs/)
* [UUID](https://crates.io/crates/uuid)
* [Chrono](https://crates.io/crates/chrono)
* [Dotenvy](https://crates.io/crates/dotenvy)

---

## 🧪 Migrations com SQLx

```bash
sqlx migrate run          # Aplica as migrations
sqlx migrate add <nome>   # Cria nova migration
```

---

## ✍️ Autor

**Roberto Lima**
[🔗 GitHub](https://github.com/robertolima-dev) — [🌐 Portfólio](https://robertolima-developer.vercel.app/)
📧 [robertolima.izphera@gmail.com](mailto:robertolima.izphera@gmail.com)

---

## 📜 Licença

MIT © 2025 — Livre para uso, estudo e modificação.

