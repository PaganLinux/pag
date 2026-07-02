// Serwis autoryzacji — JWT + bcrypt
use crate::config::Config;
use crate::db::DbPool;
use crate::models::{AuthResponse, Claims, CreateUserRequest, LoginRequest, User, UserPublic};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub struct AuthService;

impl AuthService {
    /// Rejestracja nowego użytkownika
    pub async fn register(
        pool: &DbPool,
        req: CreateUserRequest,
    ) -> Result<AuthResponse, anyhow::Error> {
        // Sprawdź czy użytkownik już istnieje
        let existing = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE username = ? OR email = ?",
        )
        .bind(&req.username)
        .bind(&req.email)
        .fetch_one(pool)
        .await?;

        if existing > 0 {
            anyhow::bail!("User with this username or email already exists");
        }

        let hashed = hash(&req.password, DEFAULT_COST)?;

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password, role) VALUES (?, ?, ?, 'user') RETURNING *",
        )
        .bind(&req.username)
        .bind(&req.email)
        .bind(&hashed)
        .fetch_one(pool)
        .await?;

        let token = Self::generate_token(&user, &Config::from_env())?;

        Ok(AuthResponse {
            token,
            user: UserPublic::from(user),
        })
    }

    /// Logowanie
    pub async fn login(
        pool: &DbPool,
        req: LoginRequest,
    ) -> Result<AuthResponse, anyhow::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = ?",
        )
        .bind(&req.username)
        .fetch_optional(pool)
        .await?;

        let user = user.ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;

        if !verify(&req.password, &user.password)? {
            anyhow::bail!("Invalid credentials");
        }

        let token = Self::generate_token(&user, &Config::from_env())?;

        Ok(AuthResponse {
            token,
            user: UserPublic::from(user),
        })
    }

    /// Pobierz użytkownika po ID
    pub async fn get_user_by_id(pool: &DbPool, user_id: i64) -> Result<Option<User>, anyhow::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    /// Waliduj token JWT
    pub fn validate_token(config: &Config, token: &str) -> Result<Claims, anyhow::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }

    fn generate_token(user: &User, config: &Config) -> Result<String, anyhow::Error> {
        let now = Utc::now();
        let claims = Claims {
            sub: user.id,
            username: user.username.clone(),
            role: user.role.clone(),
            exp: (now + chrono::Duration::hours(config.jwt_expiry_hours)).timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        Ok(encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        )?)
    }
}
