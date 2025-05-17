use std::sync::Arc;

use crate::{
    auth::{authenticator::Authenticator, pass_service::PasswordService},
    usecases::{auth, note, user},
};
use dmn::repos::{note::NoteRepository, user::UserRepository};

#[derive(Clone)]
pub struct AppState {
    // Authenticator
    pub authenticator: Arc<dyn Authenticator + Send + Sync>,
    // Authentication Use Cases
    pub change_user_pass_use_case: auth::ChangeUserPasswordUseCase,
    pub auth_login_use_case: auth::AuthLoginUseCase,
    pub auth_refresh_use_case: auth::AuthRefreshUseCase,
    // Note Use Cases
    pub create_note_use_case: note::CreateNoteUseCase,
    pub delete_note_use_case: note::DeleteNoteUseCase,
    pub get_note_use_case: note::GetNoteUseCase,
    pub get_notes_use_case: note::GetNotesUseCase,
    pub update_note_use_case: note::UpdateNoteUseCase,
    // User Use Cases
    pub delete_self_user_use_case: user::DeleteSelfUserUseCase,
    pub get_self_user_use_case: user::GetSelfUserUseCase,
    pub register_user_use_case: user::RegisterUserUseCase,
    pub update_self_user_use_case: user::UpdateSelfUserUseCase,
}

impl AppState {
    pub fn new(
        authenticator: Arc<dyn Authenticator + Send + Sync>,
        pass_service: Arc<dyn PasswordService + Send + Sync>,
        note_repo: Arc<dyn NoteRepository + Send + Sync>,
        user_repo: Arc<dyn UserRepository + Send + Sync>,
    ) -> Self {
        // Authentication Use Cases
        let change_user_pass_use_case =
            auth::ChangeUserPasswordUseCase::new(user_repo.clone(), pass_service.clone());
        let auth_login_use_case = auth::AuthLoginUseCase::new(
            authenticator.clone(),
            user_repo.clone(),
            pass_service.clone(),
        );
        let auth_refresh_use_case = auth::AuthRefreshUseCase::new(authenticator.clone());
        // Note Use Cases
        let create_note_use_case = note::CreateNoteUseCase::new(note_repo.clone());
        let delete_note_use_case = note::DeleteNoteUseCase::new(note_repo.clone());
        let get_note_use_case = note::GetNoteUseCase::new(note_repo.clone());
        let get_notes_use_case = note::GetNotesUseCase::new(note_repo.clone());
        let update_note_use_case = note::UpdateNoteUseCase::new(note_repo.clone());
        // User Use Cases
        let delete_self_user_use_case =
            user::DeleteSelfUserUseCase::new(user_repo.clone(), pass_service.clone());
        let get_self_user_use_case = user::GetSelfUserUseCase::new();
        let register_user_use_case =
            user::RegisterUserUseCase::new(user_repo.clone(), pass_service.clone());
        let update_self_user_use_case = user::UpdateSelfUserUseCase::new(user_repo.clone());

        AppState {
            // Authenticator
            authenticator,
            // Authentication Use Cases
            change_user_pass_use_case,
            auth_login_use_case,
            auth_refresh_use_case,
            // Note Use Cases
            create_note_use_case,
            delete_note_use_case,
            get_note_use_case,
            get_notes_use_case,
            update_note_use_case,
            // User Use Cases
            delete_self_user_use_case,
            get_self_user_use_case,
            register_user_use_case,
            update_self_user_use_case,
        }
    }
}
