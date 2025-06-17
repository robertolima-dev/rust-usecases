# 📚 Guia Rápido - Principais Macros e Métodos do SQLx (Rust) com exemplos de uso

---

## ✅ Principais Macros de Query no SQLx

| Macro | Quando usar | Tipo de Retorno |
|---|---|---|
| `sqlx::query!` | Quando **não precisa mapear para uma struct existente** (Ex: operações simples, `INSERT`, `UPDATE`, etc) | Retorna uma struct gerada automaticamente |
| `sqlx::query_as!` | Quando quer mapear o resultado para uma **struct definida por você (ex: `User`, `Notification`)** | Retorna a sua struct customizada |
| `sqlx::query_scalar!` | Quando quer recuperar **um único valor escalar** (Ex: `COUNT(*)`, `MAX()`, `SUM()`, etc) | Ex: `i64`, `String`, etc |

---

## ✅ Exemplos práticos de uso das Macros:

### 📌 query!

```rust
sqlx::query!(
    r#"
    UPDATE users SET name = $1 WHERE id = $2
    "#,
    "Novo Nome",
    user_id
)
.execute(&db)
.await?;
````

---

### 📌 query\_as!

```rust
#[derive(sqlx::FromRow)]
struct User {
    id: Uuid,
    name: String,
}

let users = sqlx::query_as!(
    User,
    r#"
    SELECT id, name FROM users
    "#
)
.fetch_all(&db)
.await?;
```

---

### 📌 query\_scalar!

```rust
let count: i64 = sqlx::query_scalar!(
    r#"
    SELECT COUNT(*) as "count!"
    FROM users
    "#
)
.fetch_one(&db)
.await?;
```

---

## ✅ Principais Métodos de Executor (`.fetch_*`, `.execute()`, etc):

| Método                 | O que faz                                                                      | Quando usar                                |
| ---------------------- | ------------------------------------------------------------------------------ | ------------------------------------------ |
| `.execute(&db)`        | Executa queries que **não retornam linhas** (Ex: `INSERT`, `UPDATE`, `DELETE`) | ✅ Para comandos de escrita                 |
| `.fetch_one(&db)`      | Retorna **exatamente uma linha**. Erra se vier 0 ou mais de 1                  | ✅ Quando espera 1 resultado                |
| `.fetch_optional(&db)` | Retorna **`Option<T>`**: `Some(result)` ou `None` se não tiver                 | ✅ Quando pode ou não existir resultado     |
| `.fetch_all(&db)`      | Retorna **`Vec<T>`** com todas as linhas                                       | ✅ Para listas                              |
| `.fetch(&db)`          | Retorna um **Stream de resultados** (lazy fetch)                               | ✅ Para streams ou grandes volumes de dados |

---

## ✅ Exemplo prático dos métodos:

| Exemplo             | Código                                                                                                                |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `.execute()`        | `sqlx::query!("DELETE FROM users WHERE id = $1", user_id).execute(&db).await?;`                                       |
| `.fetch_one()`      | `let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id).fetch_one(&db).await?;`               |
| `.fetch_optional()` | `let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id).fetch_optional(&db).await?;`          |
| `.fetch_all()`      | `let users = sqlx::query_as!(User, "SELECT * FROM users").fetch_all(&db).await?;`                                     |
| `.fetch()`          | `let mut rows = sqlx::query!("SELECT * FROM users").fetch(&db); while let Some(row) = rows.try_next().await? { ... }` |

---

## ✅ Dicas Rápidas:

| Situação                                        | Melhor escolha                     |
| ----------------------------------------------- | ---------------------------------- |
| Query que altera o banco (Insert/Update/Delete) | `.execute()`                       |
| Contagem (`COUNT(*)`, etc)                      | `query_scalar!()` + `.fetch_one()` |
| Consulta única obrigatória                      | `.fetch_one()`                     |
| Consulta que pode não existir                   | `.fetch_optional()`                |
| Consulta de lista                               | `.fetch_all()`                     |
| Streams de grandes volumes                      | `.fetch()`                         |

---

## ✅ Sobre `?`, `.map_err()` e Error Handling:

| Técnica            | Quando usar                                    |                       |                              |
| ------------------ | ---------------------------------------------- | --------------------- | ---------------------------- |
| `.await?` direto   | Se quiser propagar o erro                      |                       |                              |
| \`.await.map\_err( | e                                              | AppError::from(e))?\` | Se quiser transformar o erro |
| `match`            | Se quiser logar ou tratar de forma customizada |                       |                              |

---

## ✅ Extras:

👉 Todas essas macros fazem **verificação de SQL em tempo de compilação**, desde que você tenha o **feature `offline` configurado no Cargo.toml**.

```toml
[dependencies.sqlx]
version = "0.7"
features = ["postgres", "runtime-tokio", "macros"]
```

---

## ✅ Conclusão:

Use:

✔️ **`query!` → Queries simples, sem structs customizadas**
✔️ **`query_as!` → Queries que populam structs suas**
✔️ **`query_scalar!` → Quando só precisa de um valor escalar**
✔️ **`.fetch_*()` → Dependendo do resultado que você espera**

---

**By: Roberto Lima (Rust Usecases 🦀)**
