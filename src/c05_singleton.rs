#[derive(PartialEq)]
pub enum Triple {
    ALPHA, BETA, GAMMA
}


impl Triple {
    pub fn get_instance(string: &str) -> Triple {
        match string {
            "ALPHA" => Triple::ALPHA,
            "BETA" => Triple::BETA,
            _ => Triple::GAMMA,
        }
    }
}
