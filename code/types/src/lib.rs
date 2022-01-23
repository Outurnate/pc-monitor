#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![forbid(unsafe_code)]

use core::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Sample
{
  cpu: heapless::Vec<u16, 256>,
  mem: u16,
  load: u16
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Message
{
  SetInterval(Duration)
}

#[cfg(feature = "std")]
impl Sample
{
  pub fn new(cpu: Vec<systemstat::CPULoad>, mem: systemstat::Memory, load: systemstat::LoadAverage) -> Self
  {
    let mem: u64 = (mem.total.as_u64() - mem.free.as_u64()) / 1024 / 1024;
    Self
    {
      cpu: cpu.into_iter().map(|cpu| ((1.0 - cpu.idle) * (u16::MAX as f32)) as u16).collect::<heapless::Vec<u16, 256>>(),
      mem: mem as u16,
      load: (load.one * 10.0) as u16
    }
  }
}