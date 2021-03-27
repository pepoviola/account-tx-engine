#[derive(Debug)]
pub struct Amount {
    pub inner: u64,
}

impl Amount {
    pub fn new(inner: u64) -> Amount {
        Amount { inner }
    }

    pub fn value(&self) -> u64 {
        self.inner
    }

    pub fn from_input(amount: f64) -> Amount {
        let inner = (amount * 1.0e+4_f64) as u64;
        Amount { inner }
    }

    pub fn to_output(&self) -> f64 {
        (self.inner as f64 / 1.0e+4_f64) as f64
    }
}