#![feature(llvm_asm)]
#![forbid(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;
use postcard::{CobsAccumulator, FeedResult};
use types::Sample;
use avr_device::atmega328p::Peripherals;

const CPU_FREQUENCY_HZ: u64 = 16_000_000;
const BAUD: u64 = 9600;
const UBRR: u16 = (CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;

struct Dirtyable<T>
{
  value: T,
  dirty: bool
}

impl<T> Dirtyable<T>
{
  fn new(value: T) -> Self
  {
    Self
    {
      value,
      dirty: true
    }
  }

  fn dirty(&mut self, value: T)
  {
    self.dirty = true;
    self.value = value;
  }

  fn clean(&mut self) -> Option<&T>
  {
    if self.dirty
    {
      self.dirty = false;
      Some(&self.value)
    }
    else
    {
      None
    }
  }
}

fn feed_stats(sample: Sample)
{
}

#[no_mangle]
pub extern fn main()
{
  let dp = Peripherals::take().unwrap();
  Serial::new(UBRR)
    .character_size(CharacterSize::EightBits)
    .mode(Mode::Asynchronous)
    .parity(Parity::Disabled)
    .stop_bits(StopBits::OneBit)
    .configure();
  
  let mut cobs_buf = CobsAccumulator::<256>::new();
  while let Some(element) = try_receive()
  {
    let mut window: &[u8] = &[element];
    'cobs: while !window.is_empty()
    {
      window = match cobs_buf.feed(window)
      {
        FeedResult::Consumed => break 'cobs,
        FeedResult::OverFull(next) => next,
        FeedResult::DeserError(next) => next,
        FeedResult::Success { data, remaining } =>
        {
          feed_stats(data);
          remaining
        }
      };
    }
  }
}