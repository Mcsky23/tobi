use std::fs;

use crate::settings;
use crate::ctf;

pub fn get_context() -> Result<(ctf::Ctf, ctf::challenge::Challenge), std::io::Error> {
    let mut context = String::new();
    let mut file = fs::File::open(settings::CONTEXT_FILE).unwrap();
    let buf = fs::read_to_string(settings::CONTEXT_FILE).unwrap();
    let aux = buf.trim().split(":").collect::<Vec<&str>>();
    let ctf_id = aux[0].to_string();
    let chall_id = aux[1].to_string();

    let ctf = crate::db::get_ctf_from_id(ctf_id).unwrap();
    let challenge = crate::db::get_challenge_from_id(chall_id).unwrap();
    Ok((ctf, challenge))
}

pub fn save_context() {
    todo!();
}