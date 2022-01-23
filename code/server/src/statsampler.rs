use std::io;
use systemstat::{System, Platform, DelayedMeasurement, CPULoad};
use types::Sample;
use core::time::Duration;
use tokio::time::{sleep, Interval, interval, MissedTickBehavior};

pub struct StatSampler
{
  sys: System,
  last_cpu_measurement: DelayedMeasurement<Vec<CPULoad>>,
  sampler: Interval
}

impl StatSampler
{
  pub async fn new(sampler_interval: Duration) -> io::Result<StatSampler>
  {
    let sys = System::new();
    let cpu = sys.cpu_load()?;
    let mut sampler = interval(sampler_interval);
    sampler.set_missed_tick_behavior(MissedTickBehavior::Skip);
    sleep(sampler_interval).await;
    Ok(Self
    {
      sys,
      last_cpu_measurement: cpu,
      sampler
    })
  }

  pub async fn wait_next_sample(&mut self) -> io::Result<Sample>
  {
    self.sampler.tick().await;
    let cpu = self.last_cpu_measurement.done()?;
    self.last_cpu_measurement = self.sys.cpu_load()?;
    let mem = self.sys.memory()?;
    let load = self.sys.load_average()?;

    let sample = Sample::new(cpu, mem, load);
    Ok(sample)
  }

  pub fn set_interval(&mut self, sampler_interval: Duration)
  {
    self.sampler = interval(sampler_interval);
    self.sampler.set_missed_tick_behavior(MissedTickBehavior::Skip);
  }
}