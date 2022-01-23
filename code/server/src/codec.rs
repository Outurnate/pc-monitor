use std::io;
use bytes::BytesMut;
use log::warn;
use postcard::{CobsAccumulator, FeedResult, to_stdvec_cobs};
use tokio_util::codec::{Encoder, Decoder};
use types::{Sample, Message};
use color_eyre::eyre::Result;

pub struct Codec
{
  cobs_buf: CobsAccumulator<256>
}

impl Codec
{
  pub fn new() -> Self
  {
    Self
    {
      cobs_buf: CobsAccumulator::new()
    }
  }
}

impl Encoder<Sample> for Codec
{
  type Error = io::Error;

  fn encode(&mut self, item: Sample, dst: &mut BytesMut) -> Result<(), Self::Error>
  {
    match to_stdvec_cobs(&item)
    {
      Ok(buffer) => dst.extend_from_slice(&buffer),
      Err(error) => warn!("Error encoding message: {}", error),
    }
    Ok(())
  }
}

impl Decoder for Codec
{
  type Item = Message;
  type Error = io::Error;

  fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>
  {
    let mut result = None;
    let mut window = &src[..];
    'cobs: while !window.is_empty()
    {
      window = match self.cobs_buf.feed(&window)
      {
        FeedResult::Consumed => break 'cobs,
        FeedResult::OverFull(next) => next,
        FeedResult::DeserError(next) => next,
        FeedResult::Success { data, remaining } =>
        {
          result = Some(data);
          remaining
        }
      }
    }
    Ok(result)
  }
}