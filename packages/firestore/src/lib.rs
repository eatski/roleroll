
use std::{collections::HashMap};
use future::{FireStoreResource};
use json_bridge::{set_document_field};
use serde::{Serialize, Deserialize};

mod js_bridge;
pub mod json_bridge;
pub mod future;

#[derive(Serialize, Deserialize)]
pub struct MemberInput {
    pub name: String,
    pub is_host: bool,
}

impl FireStoreResource for MemberInput {
    type ParamForPath = String;

    fn path(param: &Self::ParamForPath) -> String {
        format!("{}/rooms/{}/members",NAME_SPACE,param)
    }
}

pub type UserToRole = HashMap<String,String>;

#[derive(Serialize, Deserialize,Clone)]
pub struct Roll {
    pub seq_num: usize,
    pub user_to_role: UserToRole,
}

impl FireStoreResource for Roll {
    fn path(room_id: &String) -> String {
        format!("{}/rooms/{}/rolls",NAME_SPACE,room_id)
    }
    type ParamForPath = String;
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Room {
    pub rule: Option<Rule>,
    pub can_join: bool,
}

impl FireStoreResource for Room {
    fn path(_: &()) -> String {
        format!("{}/rooms",NAME_SPACE)
    }
    type ParamForPath = ();
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub number: usize
}
#[derive(Serialize, Deserialize,Clone)]
pub struct Rule {
    pub roles: Vec<Role>,
}

const NAME_SPACE: &str = "rollrole/v1";
#[derive(Serialize, Deserialize,Clone)]
pub struct MemberJSON {
    pub name: String,
    pub id: String,
    pub is_host: bool,
}

impl FireStoreResource for MemberJSON {
    fn path(room_id: &String) -> String {
        format!("{}/rooms/{}/members",NAME_SPACE,room_id)
    }
    type ParamForPath = String;
}

pub fn set_rule(room_id: &str,rule: &Rule, on_complete: impl FnOnce() + 'static, on_error: impl FnOnce() + 'static) {
    let path: &str = &format!("{}/rooms/{}",NAME_SPACE,room_id);
    set_document_field(path,"rule",rule,on_complete,on_error);
}

pub fn set_can_join_false(room_id: &str,on_complete: impl FnOnce() + 'static, on_error: impl FnOnce() + 'static) {
    let path: &str = &format!("{}/rooms/{}",NAME_SPACE,room_id);
    set_document_field(path,"can_join",&false,on_complete,on_error);
}