#[allow(unused_variables)]
use std::collections::HashMap;

struct Locked;
struct Unlocked;

// PasswordManager<Locked> != PasswordManager<Unlocked>
struct PasswordManager<State = Locked> {
    master_password: String,
    passowrds: HashMap<String, String>,
    state: std::marker::PhantomData<State>,
}

impl PasswordManager<Locked> {
    pub fn unlock(self, master_pass: String) -> PasswordManager<Unlocked> {
        PasswordManager {
            master_password: self.master_password,
            passowrds: self.passowrds,
            state: std::marker::PhantomData::<Unlocked>,
        }
    }
}

impl PasswordManager<Unlocked> {
    pub fn lock(self) -> PasswordManager<Locked> {
        PasswordManager {
            master_password: self.master_password,
            passowrds: self.passowrds,
            state: std::marker::PhantomData::<Locked>,
        }
    }

    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.passowrds
    }
    pub fn add_password(&mut self, username: String, password: String) {
        self.passowrds.insert(username, password);
    }
}

impl<State> PasswordManager<State> {
    pub fn encription(&self) -> String {
        todo!();
    }
    pub fn version(&self) -> String {
        todo!();
    }
}

impl PasswordManager<Locked> {
    pub fn new(master_password: String) -> Self {
        PasswordManager {
            master_password,
            passowrds: Default::default(),
            state: Default::default(),
        }
    }
}
fn main() {
    let manager = PasswordManager::new("passABC".to_owned());
    let manager = manager.unlock("passABC".to_owned());
    manager.list_passwords();
    manager.lock();
}
