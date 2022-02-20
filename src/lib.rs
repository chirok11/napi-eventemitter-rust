#![deny(clippy::all)]

use napi::{JsFunction, threadsafe_function::{ThreadsafeFunction, ErrorStrategy, ThreadsafeFunctionCallMode}};
use napi::threadsafe_function::ThreadSafeCallContext;
use napi::bindgen_prelude::*;
use serde::{Serialize, Deserialize};
use futures_util::StreamExt;
use log::{debug, LevelFilter, trace};
use std::io::Write;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
#[derive(Serialize, Deserialize, Clone)]
struct DownloadProgress {
  pub target: &'static str,
  pub downloaded: i64,
  pub total: Option<i64>,
}

pub trait ReqwestExt<T> {
  fn napify(self) -> napi::Result<T>;
}

impl ReqwestExt<reqwest::Response> for std::result::Result<reqwest::Response, reqwest::Error> {
  fn napify(self) -> napi::Result<reqwest::Response> {
    match self {
      Ok(t) if matches!(
        t.status(),
        reqwest::StatusCode::NOT_FOUND | reqwest::StatusCode::FORBIDDEN
      ) => Err(napi::Error::from_reason(format!("Response status code is invalid: {:?}", t.status()))),
      Ok(t) => Ok(t),
      Err(e) => Err(napi::Error::from_reason(format!("{}", e)))
    }
  }
}

#[napi]
struct FileDownloader {
  emitter: Option<ThreadsafeFunction<DownloadProgress, ErrorStrategy::Fatal>>
}

#[napi]
impl FileDownloader {
  #[napi(constructor)]
  pub fn new(emitter: Option<JsFunction>) -> Self {
    if let Some(func) = emitter {
      let tsfn: ThreadsafeFunction<_, ErrorStrategy::Fatal> = func.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<DownloadProgress>| {
          Ok(vec![ctx.env.create_string(ctx.value.target)?.into_unknown(), ctx.env.to_js_value(&ctx.value)?.into_unknown()])
      }).unwrap();
      Self {
        emitter: Some(tsfn)
      }
    } else {
      Self {
        emitter: None
      }
    }
  }

  #[napi]
  pub async fn download_file(&mut self, url: String, filename: String) -> Result<()> {
    let res = reqwest::Client::new();
    debug!("fetching {}", &url);
    let res = res.get(url).send().await.napify()?;
    let length = res.content_length().map(|v| v as i64);
    let mut stream = res.bytes_stream();

    let emit = self.emitter.clone().unwrap();
    debug!("creating file {}", &filename);
    let mut file = File::create(&filename).await?;
    let mut downloaded = 0;

    while let Some(item) = stream.next().await {
      let chunk = item.unwrap();
      file.write_all(&chunk).await?;
      downloaded += chunk.len() as u64;
      debug!("downloaded: {}", downloaded);
      emit.call(DownloadProgress {
        target: "progress",
        downloaded: downloaded as i64,
        total: length
      }, ThreadsafeFunctionCallMode::NonBlocking);
    }

    emit.call(DownloadProgress {
      target: "progress",
      downloaded: downloaded as i64,
      total: length
    }, ThreadsafeFunctionCallMode::NonBlocking);

    Ok(())
  }
}

#[napi]
fn setup_log() {
  pretty_env_logger::init();
}