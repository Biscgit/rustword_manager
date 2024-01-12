#[derive(Clone)]
pub struct LoginStates {
    pub state: LoginState,
    last_password: Option<String>,
}

impl LoginStates {
    pub fn new() -> LoginStates {
        LoginStates {
            state: LoginState::Login,
            last_password: None,
        }
    }

    pub fn check_pw(self, password: &String) -> bool {
        if let Some(last_password) = self.last_password {
            return last_password == *password;
        }
        false
    }

    pub fn set_password(&mut self, password: String) {
        self.last_password = Some(password);
    }
}

#[derive(PartialEq, Clone)]
pub enum LoginState {
    Login,
    IncorrectLogin,

    Register,
    NewVaultConfirmMatch,
    NewVaultConfirmNoMatch,

    Unlocked,
}
