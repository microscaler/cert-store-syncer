#![allow(unused, unused_imports, unused_variables)]
use anyhow::{anyhow, bail, Context, Error, Result};
use aws_sdk_s3::{config, types::ByteStream, Client, Credentials, Region};

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

// list buckets
async fn list_buckets(client: &Client, bucket_name: &str) -> Result<Vec<String>> {
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

// upload file/s
async fn upload_file(client: &Client, bucket_name: &str, key: &str, file_path: &Path) -> Result<()> {
    // check if file exists
    if !file_path.exists() {
        bail!("File does not exist: {}", file_path.display());
    }

    // prepare byte stream
    let body = ByteStream::from_path(&file_path).await?;
    let content_type = mime_guess::from_path(&file_path)
        .first_or_octet_stream()
        .to_string();

    // build - aws request
    let req = client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body)
        .content_type(content_type);

    // send request
    let resp = req.send().await?;

    // check response
    if Err(resp) != Ok(()) {
        bail!("Failed to upload file");
    }

    Ok(())
}

// download file
async fn download_file(client: &Client, bucket_name: &str, key: &str, dir: &Path) -> Result<()> {
    // validate file path
    if !dir.is_dir() {
        bail!("Invalid file path: {}", dir.display());
    }

    // create file path and parent dir(s)
    let file_path = dir.join(key);
    let parent_dir = file_path
        .parent()
        .ok_or_else(|| anyhow!("Failed to get parent dir for {:?}", file_path))?;
    if !parent_dir.exists() {
        create_dir_all(parent_dir)?;
    }

    // build request
    let req = client.get_object().bucket(bucket_name).key(key);

    // execute request
    let resp = req.send().await?;

    // stream response to disk
    let mut data: ByteStream = resp.body;
    let file = File::create(&file_path)?;
    let mut buf_writter = BufWriter::new(file);
    while let Some(chunk) = data.next().await {
        let chunk = chunk?;
        buf_writter.write_all(&chunk)?;
    }
    buf_writter.flush()?;
    drop(buf_writter); // close file handle correctly

    Ok(())
}
