# 🦀 Rust API - Users Service com Actix Web + PostgreSQL

Este projeto é uma API RESTful desenvolvida em **Rust** usando o poderoso framework **Actix Web**, com foco em segurança, desempenho e organização.  
A API é modular, testável e extensível — com suporte a autenticação via JWT, middleware de proteção, criação automática de usuários e perfil, além de arquitetura versionada de rotas.

---

## ✅ Funcionalidades implementadas

- ✅ Cadastro de usuário com geração de `username` baseado no e-mail (`POST /api/v1/users/`)
- ✅ Autenticação com JWT (`POST /api/v1/login/`)
- ✅ Endpoint protegido com middleware JWT (`GET /api/v1/me/`)
- ✅ Profile associado automaticamente ao criar um usuário
- ✅ Criptografia de senha com Bcrypt
- ✅ Integração com PostgreSQL
- ✅ Migrations com `sqlx migrate`
- ✅ Hot reload com `cargo-watch`
- ✅ Modularização da aplicação por pastas e versões de API
- ✅ Middleware de autenticação
- ✅ Token com tempo de expiração (`JWT_EXPIRES_IN`)
- ✅ Suporte a `.env` com `dotenvy`

---

## 🗂️ Estrutura de Pastas

```bash
src/
├── main.rs                # Entry point da aplicação
├── db/                    # Conexão e utilitários para banco de dados
├── middleware/            # Middlewares como JWT Auth
├── models/                # User, Profile, Request, Response etc
├── routes/                # Rotas organizadas por versão (v1, v2...)
│    ├── user_routes.rs
│    ├── auth_routes.rs
│    └── configure.rs      # Agrupa e configura as rotas v1
├── services/              # Regras de negócio (user_service, auth_service...)
````

---

## 🔐 Autenticação com JWT

* O login (`POST /api/v1/login/`) retorna o `UserResponse` com token
* Endpoints privados requerem header:

```
Authorization: Token {token}
```

* O token é validado no middleware antes de permitir o acesso
* Tempo de expiração configurável no `.env` com `JWT_EXPIRES_IN`

---

## 🧪 Endpoints disponíveis

| Método | Rota             | Descrição                     | Protegido? |
| ------ | ---------------- | ----------------------------- | ---------- |
| POST   | `/api/v1/users/` | Criar usuário + profile       | ❌ Não      |
| POST   | `/api/v1/login/` | Login e geração de token      | ❌ Não      |
| GET    | `/api/v1/me/`    | Perfil do usuário autenticado | ✅ Sim      |

---

## 📥 Exemplo de criação de usuário

```json
{
  "email": "roberto@email.com",
  "first_name": "Roberto",
  "last_name": "Lima",
  "password": "senha123"
}
```

> 🧠 O `username` será gerado automaticamente com base no e-mail, como `roberto_email_com`.

---

## 🛠️ Como rodar localmente

### 1. Clonar o projeto

```bash
git clone https://github.com/robertolima-dev/rust-usecases.git
cd rust-usecases
```

### 2. Configurar o arquivo `.env`

```env
DATABASE_URL=postgres://usuario:senha@localhost:5432/nome_do_banco
JWT_SECRET=sua_chave_ultra_secreta
JWT_EXPIRES_IN=86400
```

### 3. Rodar as migrations + iniciar servidor

```bash
./dev.sh
```

> O script roda `.env`, aplica as migrations e inicia com hot reload via `cargo watch`.

---

## 📦 Migrations com SQLx

A estrutura `migrations/` armazena os arquivos `.sql`.

```bash
sqlx migrate add nome_da_migration
sqlx migrate run
```

---

## ⚙️ Dependências principais

* [actix-web](https://actix.rs/) - Framework web assíncrono
* [sqlx](https://docs.rs/sqlx/) - Driver PostgreSQL e ferramenta de migration
* [bcrypt](https://crates.io/crates/bcrypt) - Hash de senhas
* [jsonwebtoken](https://crates.io/crates/jsonwebtoken) - JWT para autenticação
* [uuid](https://crates.io/crates/uuid) - Identificadores únicos
* [chrono](https://crates.io/crates/chrono) - Datas e horários
* [serde](https://serde.rs/) - Serialização/Deserialização

---

## 🔐 Próximos passos

* Refresh token 🔁
* Logout com blacklist 🛑
* Paginação e ordenação 📄
* Upload para S3 ☁️
* Integração com SQS/SES/Kafka 💬

---

## ✍️ Autor

**Roberto Lima**
[🔗 GitHub](https://github.com/robertolima-dev) — [🌐 Portfólio](https://robertolima-developer.vercel.app)
📧 [robertolima.izphera@gmail.com](mailto:robertolima.izphera@gmail.com)

---

## 📜 Licença

MIT © 2025 — Livre para uso, estudo e evolução 🔥

