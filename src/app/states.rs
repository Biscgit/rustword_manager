#[derive(Clone)]
pub struct LoginStates {
    // stores the current login state and confirm password for registration
    pub state: LoginState,
    last_password: Option<String>,
}

impl LoginStates {
    pub fn new() -> LoginStates {
        // create a default state
        LoginStates {
            state: LoginState::Login,
            last_password: None,
        }
    }

    pub fn check_pw(self, password: &String) -> bool {
        // checks confirmation password on vault creation
        if let Some(last_password) = self.last_password {
            return last_password == *password;
        }
        false
    }

    pub fn set_password(&mut self, password: String) {
        // sets first entered password
        self.last_password = Some(password);
    }

    pub fn clear_password(&mut self) {
        // clears first entered password
        self.last_password = None;
    }
}

#[derive(PartialEq, Clone)]
pub enum LoginState {
    // enum holding all possible application states
    // rendering and input handling depend on these
    Login,
    IncorrectLogin,

    Register,
    NewVaultConfirmMatch,
    NewVaultConfirmNoMatch,

    Unlocked,
}
