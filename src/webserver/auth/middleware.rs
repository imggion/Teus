use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    error::ErrorUnauthorized,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::{
    future::{Future, Ready, ready},
    pin::Pin,
    rc::Rc,
};

// JWT claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
    pub id: i32,    // User ID
}

pub struct AuthMiddlewareFactory {
    jwt_secret: String,
}

impl AuthMiddlewareFactory {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

// Middleware che sarà eseguito per ogni richiesta
pub struct AuthMiddleware<S> {
    service: Rc<S>,
    jwt_secret: String,
}

// Implementazione della factory per il middleware
impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
            jwt_secret: self.jwt_secret.clone(),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let jwt_secret = self.jwt_secret.clone();

        Box::pin(async move {
            // Estrai il token dal header Authorization
            let auth_header = req.headers().get("Authorization");
            let token = match auth_header {
                Some(header) => {
                    let header_str = header
                        .to_str()
                        .map_err(|_| ErrorUnauthorized("Invalid Authorization header format"))?;

                    if !header_str.starts_with("Bearer ") {
                        return Err(ErrorUnauthorized("Invalid Authorization header format"));
                    }

                    header_str[7..].trim()
                }
                None => return Err(ErrorUnauthorized("Authorization header missing")),
            };

            // Decodifica JWT
            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::new(Algorithm::HS256),
            )
            .map_err(|_| ErrorUnauthorized("Invalid token"))?;

            println!("Token decoded successfully: {:?}", token_data);

            // Estrai claims
            let claims = token_data.claims;

            // Verifica scadenza (già fatto da jsonwebtoken)

            // Inserisci i claims nelle estensioni della richiesta
            req.extensions_mut().insert(claims.clone());

            println!("Claims extracted: {:?}", claims);

            // Procedi con il prossimo middleware/handler
            service.call(req).await
        })
    }
}
