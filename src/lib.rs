
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Currency {
    pub code: &'static str,
    pub symbol: &'static str,
    pub description: &'static str,
    pub rate: f64,
}

pub struct Bitcoin {
    pub usd: Currency,
    pub gbp: Currency,
    pub eur: Currency,
}

impl Bitcoin {
    pub fn get() -> Self {

    }
}