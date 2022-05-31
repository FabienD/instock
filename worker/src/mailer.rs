use anyhow::Result;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::AsyncSmtpTransportBuilder;
use lettre::{AsyncSmtpTransport, AsyncStd1Executor};

pub fn init_mailer(
    host: String,
    port: u16,
    user: String,
    password: String,
) -> Result<AsyncSmtpTransport<AsyncStd1Executor>> {
    let mailer: AsyncSmtpTransportBuilder;

    if !user.is_empty() && !password.is_empty() {
        let credentials = Credentials::new(user.to_string(), password.to_string());
        let tls_parameters = TlsParameters::new(host.to_owned().into())?;

        mailer = AsyncSmtpTransport::<AsyncStd1Executor>::builder_dangerous(&host)
            .port(port)
            .credentials(credentials)
            .tls(Tls::Required(tls_parameters));

    // For local SMTP, not secure, no tls.
    } else {
        mailer = AsyncSmtpTransport::<AsyncStd1Executor>::builder_dangerous(&host).port(port)
    };

    Ok(mailer.build())
}
