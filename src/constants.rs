pub enum UserState {
    Default,
    Blocking
}

impl UserState {
    pub fn new(state: i64) -> Option<Self> {
        if state == 0 {
            return Some(UserState::Default)
        } else if state == 1 {
            return Some(UserState::Blocking)
        }
        return None
    }

    pub fn state(&self) -> i64 {
        match self {
            UserState::Default => 0,
            UserState::Blocking => 1,
        }
    }
}
