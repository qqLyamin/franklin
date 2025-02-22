use std::collections::BTreeMap;
use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, Error, error::ErrorUnauthorized, web};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::time::SystemTime;
use crate::internal::contracts::{Service, UserRepo};

// todo: check cookie
pub async fn jwt<R: UserRepo + 'static>(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let service = req.app_data::<web::Data<Service<R>>>().unwrap();
    let check = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .and_then(|token| {
            let key: Hmac<Sha256> = Hmac::new_from_slice(service.secret.as_bytes()).unwrap();
            let claims: Result<BTreeMap<String, String>, _> = token.verify_with_key(&key);
            claims.ok()
        })
        .and_then(|claims: BTreeMap<String, String>| {
            let exp = claims.get("exp")?;
            let exp = u64::from_str_radix(exp, 10).ok()?;
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if now < exp { Some(()) } else { None }
        });
    match check {
        Some(_) => next.call(req).await,
        _ => Err(ErrorUnauthorized("Invalid JWT"))
    }
}
