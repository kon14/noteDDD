use crate::auth::context::AuthAccessContext;
use common::error::AppError;
use dmn::entities::user::User;

#[derive(Clone)]
pub struct GetSelfUserUseCase;

impl GetSelfUserUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(&self, auth_ctx: AuthAccessContext) -> Result<User, AppError> {
        let user = auth_ctx.user;
        Ok(user)
    }
}
