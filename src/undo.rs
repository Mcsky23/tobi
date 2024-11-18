// Module that handles undoing action
// It is able to undo the last action by saving a log of the last action in a tmp file
// 

use std::{fs::File, io::Write};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::ctf::challenge::remove_chall;
use crate::db;
use std::fs;
use crate::context;
use crate::settings;

#[derive(Serialize, Deserialize)]
pub struct UndoAction {
    action: String,
    args: Vec<String>,
}

impl UndoAction {
    pub fn new(action: String, args: Vec<String>) -> Self {
        UndoAction {
            action,
            args,
        }
    }

    pub fn log_action(self) {
        let mut file = File::create("/tmp/tobi").unwrap_or_else(|_| {
            println!("Could not create log file");
            std::process::exit(1);
        });

        let serialized = serde_json::to_string(&self).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    // functions that create an UndoAction entry based on the last action
    pub fn new_dir_change() -> Self {
        // get current directory
        let cur_dir = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        UndoAction::new("cd".to_string(), vec![cur_dir])
    }

    fn undo_dir_change(&self) {
        let new_dir = &self.args[0];
        println!("CHANGE_DIR: {}", new_dir);

        println!("Changed dir back to {}", new_dir);
    }

    pub fn new_ctf_create(name: &String) -> Self {
        UndoAction::new("ctf_new".to_string(), vec![name.clone()])
    }

    fn undo_ctf_create(&self) {
        let workdir = settings::SETTINGS.lock().unwrap().workdir.clone();
        let name = &self.args[0];
        // remove ctf from db
        db::remove_ctf(&db::get_conn(), name);
        context::save_context(None, None);

        // remove ctf directory
        let file_path = workdir + "/" + name;
        fs::remove_dir_all(file_path).unwrap();

        println!("Removed CTF {}", name);
    }

    pub fn new_chall_create(name: &String) -> Self {
        let ctf_name = context::get_context().0.unwrap().metadata.name.clone();
        UndoAction::new("chall_new".to_string(), vec![ctf_name, name.clone()])
    }

    fn undo_chall_create(&self) {
        let ctf_name = &self.args[0];
        let chall_name = &self.args[1];
        remove_chall(ctf_name, chall_name);
    }

    pub fn new_chall_solve(ctf_name: &String, chall_name: &String) -> Self {
        UndoAction::new("chall_solve".to_string(), vec![ctf_name.clone(), chall_name.clone()])
    }

    fn undo_chall_solve(&self) {
        let ctf_name = &self.args[0];
        let chall_name = &self.args[1];

        let conn = db::get_conn();
        db::set_chall_flag(&conn, ctf_name, chall_name, &"".to_string());

        println!("Unsolve challenge {} in CTF {}", chall_name, ctf_name);
    }

    pub fn new_chall_unsolve(ctf_name: &String, chall_name: &String, flag: &String) -> Self {
        UndoAction::new("chall_unsolve".to_string(), vec![ctf_name.clone(), chall_name.clone(), flag.clone()])
    }

    fn undo_chall_unsolve(&self) {
        let ctf_name = &self.args[0];
        let chall_name = &self.args[1];
        let flag = &self.args[2];

        let conn = db::get_conn();
        db::set_chall_flag(&conn, ctf_name, chall_name, flag);
        println!("Restored flag for challenge {} in CTF {}", chall_name, ctf_name);
    }

    pub fn new_context_switch(ctf_name: &String, chall_name: Option<&String>) -> Self {
        let chall_name = match chall_name {
            Some(chall_name) => chall_name.clone(),
            None => "".to_string(),
        };
        UndoAction::new("context_switch".to_string(), vec![ctf_name.clone(), chall_name])
    }

    fn undo_context_switch(&self) {
        let ctf_name = &self.args[0];
        let chall_name = &self.args[1];

        context::save_context(Some(ctf_name), Some(chall_name));
        println!("Switched context back to CTF: {} Challenge: {}", ctf_name, chall_name);
    }

    pub fn new_chall_edit(name: &String, category: &String) -> Self {
        let ctf_name = context::get_context().0.unwrap().metadata.name.clone();
        UndoAction::new("chall_edit".to_string(), vec![ctf_name, name.clone(), category.clone()])
    }

    fn undo_chall_edit(&self) {
        let ctf_name = &self.args[0];
        let chall_name = &self.args[1];
        let category = &self.args[2];

        let mut challenge = context::get_context().1.unwrap();
        challenge.edit_chall(&chall_name, &category);
        context::switch_context(&ctf_name, Some(&chall_name), false);

        println!("Edited {} -> {} [{}]", ctf_name, chall_name, category);
    }

    pub fn new_ctf_edit(old_name: &String, new_name: &String) -> Self {
        UndoAction::new("ctf_edit".to_string(), vec![old_name.clone(), new_name.clone()])
    }

    fn undo_ctf_edit(&self) {
        let old_name = &self.args[0];
        let new_name = &self.args[1];
        let mut ctf = db::get_ctf_from_name(&db::get_conn(), new_name, false).unwrap();

        ctf.change_name(old_name.clone());
        context::switch_context(&ctf.metadata.name, None, false);
        println!("Restored CTF {}", old_name);
    }



}

pub fn undo() {
    // open log file
    let file = File::open("/tmp/tobi").unwrap_or_else(|_| {
        println!("Nothing to undo");
        std::process::exit(1);
    });

    let action: UndoAction = serde_json::from_reader(file).unwrap();
    match action.action.as_str() {
        "cd" => action.undo_dir_change(),
        "ctf_new" => action.undo_ctf_create(),
        "ctf_edit" => action.undo_ctf_edit(),
        "chall_new" => action.undo_chall_create(),
        "chall_solve" => action.undo_chall_solve(),
        "chall_unsolve" => action.undo_chall_unsolve(),
        "chall_edit" => action.undo_chall_edit(),
        "context_switch" => action.undo_context_switch(),
        _ => {
            println!("Unknown action");
            std::process::exit(1);
        }
    }

    // clear log file
    fs::remove_file("/tmp/tobi").unwrap();
}