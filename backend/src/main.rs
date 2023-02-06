use ic_cdk_macros::query;
use ic_cdk_macros::update;

use std::cell::RefCell;
use std::collections::BTreeMap;
use ic_cdk::export::{candid::CandidType};
use std::any::TypeId;

#[derive(CandidType)]
struct User {
    first_name: String,
    last_name: String
}

thread_local! {
    /// User name
    static USERS: RefCell<BTreeMap<ic_cdk::export::Principal, User>> = RefCell::new(BTreeMap::new());

}

#[query]
fn hello_world() -> String {
    format!("Hello {}!", ic_cdk::api::caller())
}

#[update]
fn set_user(first_name: String, last_name: String) {
    USERS.with(|users| {
        users.borrow_mut().insert(ic_cdk::api::caller(), User {first_name, last_name})
    });
}

#[query]
fn who_am_i() -> User {
    USERS.with(|users| {
        let users = users.borrow();
        match users.get(&ic_cdk::api::caller()) {
            None => User {first_name: "John".to_string(), last_name: "Doe".to_string()},
            Some(User) => User {first_name: User.first_name.clone(), last_name: User.last_name.clone()}
        }    
    })
}

fn main() {}
