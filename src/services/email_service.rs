use crate::errors::app_error::AppError;
use aws_sdk_sesv2::Client;
use aws_sdk_sesv2::config::BehaviorVersion;
use tera::{Context, Tera};

#[allow(dead_code)]
pub struct EmailService {
    ses_client: Client,
    tera: Tera,
    sender_email: String,
}

#[allow(dead_code)]
impl EmailService {
    pub async fn new(sender_email: String) -> Result<Self, AppError> {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);

        let tera = Tera::new("templates/emails/**/*").map_err(|err| {
            eprintln!("Erro ao carregar templates: {:?}", err);
            AppError::InternalError(Some("Erro no template engine".into()))
        })?;

        Ok(Self {
            ses_client: client,
            tera,
            sender_email,
        })
    }

    pub async fn send_reset_password(
        &self,
        to_email: &str,
        user_name: &str,
        reset_link: &str,
    ) -> Result<(), AppError> {
        let mut ctx = Context::new();
        ctx.insert("name", user_name);
        ctx.insert("link", reset_link);

        let body = self.tera.render("reset_password.html", &ctx).map_err(|e| {
            eprintln!("Erro ao renderizar e-mail: {}", e);
            AppError::InternalError(Some("Erro ao montar e-mail".into()))
        })?;

        self.send_html_email(to_email, "🔐 Redefina sua senha", &body)
            .await
    }

    async fn send_html_email(
        &self,
        to_email: &str,
        subject: &str,
        body_html: &str,
    ) -> Result<(), AppError> {
        let destination = aws_sdk_sesv2::types::Destination::builder()
            .to_addresses(to_email)
            .build();

        let content = aws_sdk_sesv2::types::EmailContent::builder()
            .simple(
                aws_sdk_sesv2::types::Message::builder()
                    .subject(
                        aws_sdk_sesv2::types::Content::builder()
                            .data(subject)
                            .charset("UTF-8")
                            .build()
                            .map_err(|e| {
                                eprintln!("Erro ao construir subject Content: {:?}", e);
                                AppError::InternalError(Some("Falha ao preparar o email".into()))
                            })?,
                    )
                    .body(
                        aws_sdk_sesv2::types::Body::builder()
                            .html(
                                aws_sdk_sesv2::types::Content::builder()
                                    .data(body_html)
                                    .charset("UTF-8")
                                    .build()
                                    .map_err(|e| {
                                        eprintln!("Erro ao construir subject Content: {:?}", e);
                                        AppError::InternalError(Some(
                                            "Falha ao preparar o email".into(),
                                        ))
                                    })?,
                            )
                            .build(),
                    )
                    .build(),
            )
            .build();

        self.ses_client
            .send_email()
            .from_email_address(&self.sender_email)
            .destination(destination)
            .content(content)
            .send()
            .await
            .map_err(|err| {
                eprintln!("Erro ao enviar e-mail: {:?}", err);
                AppError::InternalError(Some("Erro ao enviar e-mail".into()))
            })?;

        Ok(())
    }
}
