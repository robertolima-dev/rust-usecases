# ✅ Principais Formatos de Retorno de uma Rota Actix Web

| Tipo de Retorno                          | Quando usar                                                                                 | Exemplo                          |
| ---------------------------------------- | ------------------------------------------------------------------------------------------- | -------------------------------- |
| `impl Responder`                         | Simples, quando você mesmo faz o match e constrói o HttpResponse                            | Rápido e direto                  |
| `Result<HttpResponse, actix_web::Error>` | Quando seu service/repository retorna um `Result`, e você quer usar `?` para propagar erros | Boa para trabalhar com `.await?` |
| `Result<T, AppError>`                    | Quando você criou um erro custom (`AppError`) que implementa `ResponseError`                | Para unificar erros da API       |
| Qualquer tipo que implementa `Responder` | Tipo `Json<T>`, `String`, `&'static str`, etc                                               | Para respostas muito simples     |

---

## ✅ Exemplos de cada tipo:

---

### ✅ 1. Retorno simples: `impl Responder`

Você mesmo faz o match na mão:

```rust
#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
```

Ou com match:

```rust
#[post("/users/")]
async fn create_user() -> impl Responder {
    match some_service().await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
```

---

### ✅ 2. Com propagação de erro (usando `?`): `Result<HttpResponse, actix_web::Error>`

```rust
#[post("/users/")]
async fn create_user() -> Result<HttpResponse, actix_web::Error> {
    let user = user_service().await?;
    Ok(HttpResponse::Ok().json(user))
}
```

> Aqui o **`?` propaga erros que implementam `ResponseError`**.

---

### ✅ 3. Usando o seu `AppError`: `Result<HttpResponse, AppError>`

```rust
use crate::errors::AppError;

#[post("/users/")]
async fn create_user() -> Result<HttpResponse, AppError> {
    let user = user_service().await?;
    Ok(HttpResponse::Created().json(user))
}
```

> ✅ O **Actix sabe converter `AppError` pra HttpResponse** se ele implementa `ResponseError`.

---

### ✅ 4. Retornando tipos que implementam `Responder` direto (ex: `Json<T>`, `String`, etc):

```rust
use actix_web::web::Json;
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

#[get("/health")]
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}
```

---

### ✅ 5. Com `Result<Json<T>, actix_web::Error>`:

```rust
#[get("/user/{id}")]
async fn get_user(id: web::Path<Uuid>) -> Result<Json<UserResponse>, actix_web::Error> {
    let user = user_service_get(id.into_inner()).await?;
    Ok(Json(user))
}
```

---

## ✅ Resumo das formas mais comuns:

| Tipo                                     | Quando usar                                                      |
| ---------------------------------------- | ---------------------------------------------------------------- |
| `impl Responder`                         | Para respostas simples ou onde você faz o match na mão           |
| `Result<HttpResponse, actix_web::Error>` | Quando precisa usar `?` e seus erros implementam `ResponseError` |
| `Result<HttpResponse, AppError>`         | Quando usa um error custom unificado                             |
| `Result<Json<T>, actix_web::Error>`      | Quando quer retornar JSON direto                                 |
| `Json<T>`                                | Para respostas simples que sempre dão certo                      |

---

## ✅ Dica para um projeto grande e modular:

👉 Use **`Result<HttpResponse, AppError>` como padrão**.
👉 Isso te permite usar **`?` em todos os services**
👉 Mantém o pattern de erro unificado
👉 E permite `map_err(|e| e.error_response())` caso precise converter outros erros.

