//! Authentication middleware for JWT token validation.
//!
//! This module provides middleware components for validating JWT tokens
//! in HTTP requests. It automatically extracts and validates Bearer tokens
//! from the Authorization header, making user claims available to handlers.

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, HttpMessage,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NotAuth {
    message: String,
}

/// JWT claims structure containing user authentication information.
///
/// This structure represents the payload of a JSON Web Token (JWT) used
/// for user authentication in the Teus system. It follows JWT standard
/// claims with additional application-specific fields.
///
/// # Standard JWT Claims
///
/// - `sub` (Subject): Identifies the user the token was issued for
/// - `exp` (Expiration): Unix timestamp when the token expires
/// - `iat` (Issued At): Unix timestamp when the token was created
///
/// # Custom Claims
///
/// - `id`: Numeric user ID for database operations
///
/// # Security Considerations
///
/// - Tokens should be validated for expiration before use
/// - The signing key must be kept secure and consistent
/// - Claims should not contain sensitive information
///
/// # Examples
///
/// ```rust
/// use teus::webserver::auth::middleware::Claims;
/// use chrono::Utc;
///
/// let claims = Claims {
///     sub: "admin".to_string(),
///     exp: (Utc::now().timestamp() + 3600) as usize, // 1 hour from now
///     iat: Utc::now().timestamp() as usize,
///     id: 1,
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject - the username of the authenticated user.
    ///
    /// This field identifies which user the token belongs to and
    /// corresponds to the username stored in the database.
    pub sub: String,

    /// Expiration time as a Unix timestamp.
    ///
    /// Tokens should be rejected if the current time is after
    /// this timestamp. This prevents indefinite token reuse.
    pub exp: usize,

    /// Issued at time as a Unix timestamp.
    ///
    /// Records when the token was created, useful for token
    /// lifecycle management and security auditing.
    pub iat: usize,

    /// Numeric user ID from the database.
    ///
    /// This provides a direct reference to the user record
    /// for efficient database lookups in authenticated endpoints.
    pub id: i32,
}

/// Factory for creating authentication middleware instances.
///
/// This factory is responsible for creating `AuthMiddleware` instances
/// with the proper JWT secret configuration. It implements the Actix-web
/// `Transform` trait to integrate with the web framework's middleware system.
///
/// # Usage
///
/// The factory is typically registered once during application startup
/// and automatically creates middleware instances for each request scope.
///
/// # Examples
///
/// ```rust
/// use actix_web::App;
/// use teus::webserver::auth::middleware::AuthMiddlewareFactory;
///
/// let app = App::new()
///     .wrap(AuthMiddlewareFactory::new("your-jwt-secret".to_string()));
/// ```
pub struct AuthMiddlewareFactory {
    /// The secret key used for JWT token validation.
    ///
    /// This must match the secret used when generating tokens.
    /// Should be cryptographically secure and kept confidential.
    jwt_secret: String,
}

impl AuthMiddlewareFactory {
    /// Creates a new authentication middleware factory.
    ///
    /// # Arguments
    ///
    /// * `jwt_secret` - The secret key for JWT token validation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use teus::webserver::auth::middleware::AuthMiddlewareFactory;
    ///
    /// let factory = AuthMiddlewareFactory::new("secure-secret-key".to_string());
    /// ```
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

/// Authentication middleware that validates JWT tokens in HTTP requests.
///
/// This middleware automatically extracts Bearer tokens from the Authorization
/// header, validates them using the configured JWT secret, and makes the user
/// claims available to downstream handlers through request extensions.
///
/// # Request Processing
///
/// 1. Extracts the Authorization header from the request
/// 2. Validates the Bearer token format
/// 3. Decodes and validates the JWT using the secret key
/// 4. Injects the claims into request extensions for handler access
/// 5. Returns 401 Unauthorized for invalid or missing tokens
///
/// # Token Format
///
/// The middleware expects tokens in the standard Bearer format:
/// ```
/// Authorization: Bearer <jwt-token>
/// ```
///
/// # Error Handling
///
/// Returns `401 Unauthorized` for:
/// - Missing Authorization header
/// - Invalid header format (not starting with "Bearer ")
/// - Invalid or expired JWT tokens
/// - Tokens signed with a different secret
pub struct AuthMiddleware<S> {
    /// The wrapped service to call after successful authentication.
    service: Rc<S>,

    /// The JWT secret key for token validation.
    jwt_secret: String,
}

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

            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::new(Algorithm::HS256),
            )
            .map_err(|_| ErrorUnauthorized("Invalid token"))?;

            let claims = token_data.claims;
            req.extensions_mut().insert(claims.clone());
            service.call(req).await
        })
    }
}
