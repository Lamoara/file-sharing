use std::collections::{HashMap, HashSet};

use uuid::Uuid;

pub type Id = Uuid;
//TODO think about this one pub type File = String;

#[derive(Debug, Default)]
pub struct Sessions {
    admins: HashSet<Id>,
    users: HashMap<Id, String>,
}

impl Sessions {
    pub fn login_admin(&mut self) -> Id {
        let uuid = Uuid::new_v4();
        self.admins.insert(uuid); //FIXME This could theoretacly already exist (Extremely unlikely but you can check)
        uuid
    }

    pub fn logout_admin(&mut self, id: Id) {
        self.admins.remove(&id); //FIXME Could maybe return the bool for info
    }

    pub fn login_user(&mut self, url: String) -> Uuid {
        let uuid = Uuid::new_v4();
        self.users.insert(uuid, url); //FIXME This could theoretacly already exist (Extremely unlikely but you can check)
        uuid
    }

    pub fn logout_user(&mut self, id: Id) {
        self.users.remove(&id); //FIXME Could maybe return the option for info
    }

    pub fn logout_all(&mut self) {
        self.admins.clear();
        self.users.clear();
    }

    pub fn verify_admin(&self, id: &Id) -> Option<Id> {
        self.admins.get(id).cloned()
    }

    pub fn verify_user(&self, id: &Id) -> Option<String> {
        self.users.get(id).cloned()
    }
}
