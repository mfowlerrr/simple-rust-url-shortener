use rand::{RngExt, distr::Alphanumeric, rng};

pub fn generate_code() -> String {
    let mut rng = rng();

    (0..6).map(|_| rng.sample(Alphanumeric) as char).collect()
}
