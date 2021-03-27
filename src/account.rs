#[derive(Debug)]
pub struct Account {
    pub client: u16,
    pub available: u64,
    pub held: u64,
    pub locked: bool,
}

impl Account {
    pub fn new_with_client(client: u16) -> Account {
        Account {
            client,
            available: Default::default(),
            held: Default::default(),
            locked: false,
        }
    }
}
