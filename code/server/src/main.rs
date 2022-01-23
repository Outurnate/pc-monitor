#![forbid(unsafe_code)]

mod statsampler;
mod codec;

use codec::Codec;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use statsampler::StatSampler;
use tokio::{signal::ctrl_c};
use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Framed;
use types::Message;
use core::time::Duration;
use tokio::select;
use futures_util::{StreamExt, SinkExt};
use color_eyre::eyre::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args
{
  #[clap(short, long)]
  port: String,

  #[clap(short, long, default_value_t = 9600)]
  baud_rate: u32,

  #[clap(short, long, default_value_t = 1000)]
  default_millis: u64,

  #[clap(short, long, default_value_t = LevelFilter::Warn)]
  log: LevelFilter
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()>
{
  color_eyre::install()?;
  let args = Args::parse();
  SimpleLogger::new().with_level(args.log).init()?;

  let port = tokio_serial::new(args.port, args.baud_rate).open_native_async()?;
  let (mut writer, mut reader) = Framed::new(port, Codec::new()).split();
  let mut sampler = StatSampler::new(Duration::from_millis(args.default_millis)).await?;
  loop
  {
    select!
    {
      sample = sampler.wait_next_sample() => writer.send(sample?).await?,
      message = reader.next() => if let Some(message) = message
      {
        match message?
        {
          Message::SetInterval(interval) => sampler.set_interval(interval)
        }
      },
      _ = ctrl_c() => break
    }
  }
  Ok(())
}
