#![allow(unused, unused_imports, unused_variables)]
use anyhow::{anyhow, bail, Context, Result};
//use aws_sdk_acm::{config, Client, Credentials, Region};
use aws_sdk_s3::{config, Client, Credentials, Region};

use std::{
    env,
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use tokio_stream::StreamExt;

// AWS environment config
const ENV_CRED_KEY_ID: &str = "AWS_KEY_ID";
const ENV_CRED_KEY_SECRET: &str = "AWS_KEY_SECRET";
const BUCKET_NAME: &str = "aws-sdk-rust-test-bucket";
const REGION: &str = "eu-west-1";

// get client
fn get_aws_client(region: &str) -> Result<Client> {
    // get credentials from environment
    let key_id = env::var(ENV_CRED_KEY_ID).context("AWS_KEY_ID not set")?;
    let key_secret = env::var(ENV_CRED_KEY_SECRET).context("AWS_KEY_SECRET not set")?;

    // create credentials
    let cred = Credentials::new(key_id, key_secret, None, None, "Loaded from env");

    // create client
    let region = Region::new(region.to_string());
    let conf_builder = config::Builder::new().region(region).credentials_provider(cred);
    let conf = conf_builder.build();

    let client = Client::from_conf(conf);
    Ok(client)
}

async fn list_keys(client: &Client, bucket_name: &str) -> Result<Vec<String>> {
    // build - aws request
    let req = client.list_objects_v2().prefix("").bucket(bucket_name);

    // send request
    let resp = req.send().await?;

    // collect keys
    let keys = resp
        .contents()
        .unwrap_or_default()
        .iter()
        .filter_map(|o| o.key.as_ref())
        .map(|k| k.to_string())
        .collect::<Vec<_>>();

    Ok(keys)
}
