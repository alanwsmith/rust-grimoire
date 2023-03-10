#![allow(warnings)]
use futures::executor::block_on;
use keyring::error::Error;
use keyring::Entry;
use meilisearch_sdk::client::*;
use meilisearch_sdk::search::SearchResult;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::{
    Result as SerdeResult,
    Value,
};
use std::env;
use std::env::args;

#[derive(Serialize, Deserialize, Debug)]
struct Page {
    id: String,
    fileName: Option<String>,
}

fn main() {
    let ms_url = "http://127.0.0.1:7700/";
    let env_key = "MEILISEARCH_TEST";
    let cred_key =
        "alan--meilisearch--scratchpad--admin-key";
    let cred_user = "alan";
    let ms_key =
        get_var(env_key, cred_key, cred_user);
    let client =
        Client::new(ms_url, ms_key.unwrap());

    block_on(async move {
        let my_stuff = client
            .index("grimoire")
            .search()
            .with_limit(14)
            .with_query("osa- example")
            .execute::<Page>()
            .await
            .unwrap()
            .hits;

        let j = serde_json::to_string(&my_stuff);

        // for thing in my_stuff {
        //     println!(
        //         "{}",
        //         thing.result.fileName.unwrap()
        //     );
        // }
    })
}

fn get_var(
    envkey: &str, credkey: &str, creduser: &str,
) -> Result<String, Error> {
    if let Ok(value) = env::var(envkey) {
        Ok(value)
    }
    else {
        if let Ok(entry) =
            Entry::new(credkey, creduser)
        {
            if let Ok(value) = entry.get_password()
            {
                Ok(value)
            }
            else {
                Err(Error::NoEntry)
            }
        }
        else {
            Err(Error::NoEntry)
        }
    }
}
