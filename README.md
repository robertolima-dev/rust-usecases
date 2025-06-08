# 🚀 Rust API - Users Service com Actix Web + PostgreSQL

Este projeto é uma API RESTful simples construída em [Rust 🦀](https://www.rust-lang.org/) utilizando **Actix Web** para o backend HTTP e **PostgreSQL** como banco de dados. É um projeto modular, com foco em boas práticas, separação de responsabilidades e uso de features modernas do ecossistema Rust.

---

## ✅ Funcionalidades implementadas

- ✅ Criar usuário (`POST /api/v1/users/`)
- ✅ Listar todos os usuários (`GET /api/v1/users/`)
- ✅ Buscar usuário por ID (`GET /api/v1/users/{id}`)
- ✅ Armazenamento de dados com PostgreSQL
- ✅ Migrations com `sqlx`
- ✅ Autoload com `cargo watch`
- ✅ Estrutura modular com:
  - `routes/` (rotas organizadas)
  - `services/` (lógica de negócio)
  - `models/` (modelos de dados)
  - `db/` (configuração do banco)

---

## 🗂️ Estrutura de Pastas

```

src/
├── main.rs              # Entry point principal da aplicação
├── db/                  # Conexão com PostgreSQL (get\_db\_pool)
├── models/              # Modelos de dados (User, etc)
├── routes/              # Rotas HTTP e configuração de rotas
│   ├── configure.rs     # Centraliza .service()
│   └── user\_routes.rs   # Endpoints para /users
├── services/            # Regras de negócio (create/get user)

````

---

## 🧪 Endpoints disponíveis

| Método | Rota                     | Descrição                  |
|--------|--------------------------|----------------------------|
| GET    | `/api/v1/users/`         | Lista todos os usuários    |
| GET    | `/api/v1/users/{id}`     | Busca usuário por ID       |
| POST   | `/api/v1/users/`         | Cria um novo usuário       |

#### 📥 Exemplo de JSON para `POST /api/v1/users`

```json
{
  "name": "Roberto Lima",
  "email": "roberto@email.com",
  "phone": "11999999999",
  "birth_date": "1979-12-12",
  "password": "senha123"
}
````

---

## 🔧 Como rodar localmente

### 1. Clone o projeto

```bash
git clone https://github.com/seu-usuario/seu-projeto.git
cd seu-projeto
```

### 2. Configure o banco de dados

Crie um arquivo `.env` na raiz:

```env
DATABASE_URL=postgres://usuario:senha@localhost:5432/nome_do_banco
```

### 3. Execute as migrations + hot reload

```bash
./dev.sh
```

> Isso carrega as variáveis do `.env`, roda as migrations com `sqlx migrate run` e inicia o servidor com hot reload usando `cargo watch`.

---

## 💡 Tecnologias utilizadas

* [Rust](https://www.rust-lang.org/)
* [Actix Web](https://actix.rs/)
* [PostgreSQL](https://www.postgresql.org/)
* [SQLx](https://docs.rs/sqlx/)
* [Serde](https://serde.rs/)
* [UUID](https://crates.io/crates/uuid)
* [Chrono](https://crates.io/crates/chrono)
* [Dotenvy](https://crates.io/crates/dotenvy)

---

## 📦 Migrations com SQLx

A pasta `migrations/` armazena os arquivos `.sql` com instruções para criar as tabelas.

```bash
sqlx migrate run     # Aplica todas as migrations pendentes
sqlx migrate add nome_migracao
```

---

## ✍️ Autor

**Roberto Lima**
[🔗 GitHub](https://github.com/robertolima-dev) — [🌐 Site Pessoal](https://robertolima-developer.vercel.app/)
📧 [robertolima.izphera@gmail.com](mailto:robertolima.izphera@gmail.com)

---

## 📜 Licença

MIT © 2025 — Livre para uso, estudo e distribuição.
