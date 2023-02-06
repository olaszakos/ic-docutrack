use ic_cdk_macros::query;

#[query]
fn hello_world() -> String {
    format!("Hello {}!", ic_cdk::api::caller())
}

fn main() {}
