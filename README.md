# 🚀 Rust API - Users Service com Actix Web + PostgreSQL

Este projeto é uma API RESTful robusta desenvolvida em [Rust 🦀](https://www.rust-lang.org/), utilizando **Actix Web** como framework web e **PostgreSQL** como banco de dados. A aplicação está estruturada de forma modular com foco em boas práticas, segurança e escalabilidade.

---

## ✅ Funcionalidades Implementadas

* ✅ Criar usuário (`POST /api/v1/users/`)
* ✅ Login e geração de JWT (`POST /api/v1/login/`)
* ✅ Obter perfil autenticado (`GET /api/v1/me/`)
* ✅ Módulo de autenticação com middleware
* ✅ Middleware JWT (`Authorization: Token <JWT>`)
* ✅ Extração de `user_id` via trait: `req.user_id()?`
* ✅ Padronização de erros com `AppError`
* ✅ Repositórios para encapsular acesso ao banco de dados
* ✅ Armazenamento de dados com PostgreSQL
* ✅ Migrations com SQLx
* ✅ Hot Reload com `cargo watch`
* ✅ Estrutura modular (routes, services, models, errors, middleware)

---

## 🚀 Tecnologias

- [Rust](https://www.rust-lang.org/)
- [Actix Web](https://actix.rs/)
- [SQLx](https://github.com/launchbadge/sqlx)
- [PostgreSQL](https://www.postgresql.org/)
- [JWT](https://jwt.io/)
- [Tracing](https://github.com/tokio-rs/tracing)

---

## 📋 Pré-requisitos

- Rust (última versão estável)
- PostgreSQL
- Docker (opcional)

## 🔧 Instalação

1. Clone o repositório:
```bash
git clone https://github.com/seu-usuario/rust-usecases.git
cd rust-usecases
```

2. Configure o banco de dados:
```bash
# Crie um banco PostgreSQL
createdb rust_usecases
```

3. Configure as variáveis de ambiente:
```bash
cp .env.example .env
# Edite o arquivo .env com suas configurações
```

4. Execute as migrações:
```bash
sqlx database create
sqlx migrate run
```

5. Execute o projeto:
```bash
./start_server.sh
```
---

## 🏗️ Estrutura do Projeto

```
src/
├── config/         # Configurações da aplicação
├── db/             # Configuração do banco de dados
├── errors/         # Tratamento de erros
├── extensions/     # Extensões do Actix Web
├── middleware/     # Middlewares (Auth, Rate Limit)
├── models/         # Modelos de dados
├── repositories/   # Acesso ao banco de dados
├── routes/         # Rotas da API
├── services/       # Lógica de negócio
└── utils/          # Utilitários (JWT, Validação, Logging)
```
---

## 🌟 Funcionalidades

### Autenticação e Autorização
- Registro de usuários
- Login com JWT
- Middleware de autenticação
- Validação de tokens

### Configurações
- Configurações centralizadas com validação
- Suporte a diferentes ambientes (dev, test, prod)
- Validação de configurações obrigatórias
- Tipos fortemente tipados para configurações

### Validação de Dados
- Validação de senhas (comprimento, caracteres, números)
- Validação de emails
- Validação de documentos
- Validação de telefones
- Mensagens de erro personalizadas

### Sistema de Logs
- Logs estruturados em JSON (produção)
- Logs formatados para desenvolvimento
- Diferentes níveis de log (error, warn, info, debug)
- Campos estruturados para melhor análise
- Timestamps em formato RFC3339
- Informações de contexto (thread, arquivo, linha)

### Banco de Dados
- Conexão com PostgreSQL
- Pool de conexões
- Transações
- Queries tipadas
- Migrations

---

## 🧪 Migrations com SQLx

```bash
sqlx migrate run          # Aplica as migrations
sqlx migrate add <nome>   # Cria nova migration
```

---

## 🔍 Exemplos de Uso

### Configuração de Logs

```rust
// Em desenvolvimento
RUST_LOG=debug,rust_usecases=trace cargo run

// Em produção
RUST_LOG=info,rust_usecases=debug cargo run
```

### Validação de Dados

```rust
#[derive(Debug, Validate)]
pub struct UserRequest {
    #[validate(email(message = "Email inválido"))]
    pub email: String,

    #[validate(custom = "validate_password")]
    pub password: String,
}
```

### Configurações

```rust
#[derive(Debug, Validate)]
pub struct Settings {
    #[validate]
    pub database: DatabaseSettings,
    #[validate]
    pub jwt: JwtSettings,
    #[validate]
    pub server: ServerSettings,
    pub environment: Environment,
}
```

## 📝 Logs

O sistema de logs fornece informações detalhadas sobre o funcionamento da aplicação:

- **Inicialização**: Configurações carregadas, conexão com banco
- **Autenticação**: Tentativas de login, tokens gerados
- **Operações**: Criação/atualização de usuários, erros de validação
- **Erros**: Detalhes de exceções, stack traces

Exemplo de log em produção:
```json
{
  "timestamp": "2024-03-14T12:00:00Z",
  "level": "info",
  "message": "Usuário criado com sucesso",
  "user_id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "usuario@exemplo.com"
}
```

## 🤝 Contribuindo

1. Fork o projeto
2. Crie sua feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

---

## ✍️ Autor

**Roberto Lima**
[🔗 GitHub](https://github.com/robertolima-dev) — [🌐 Portfólio](https://robertolima-developer.vercel.app/)
📧 [robertolima.izphera@gmail.com](mailto:robertolima.izphera@gmail.com)

---

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.
