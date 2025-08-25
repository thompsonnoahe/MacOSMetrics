#![deny(clippy::all)]

pub mod metrics;
mod sources;

use napi_derive::napi;

#[napi(constructor)]
pub struct GpuMetrics {
  pub gpu_utilization: f64,
  pub temperature: f64,
  pub power_usage: f64,
  pub name: String,
  pub id: String
}

#[napi]
impl GpuMetrics {
  #[napi]
  pub fn get_metrics(interval: u32) -> napi::Result<GpuMetrics> {
    let mut sampler = metrics::Sampler::new()
      .map_err(|e| napi::Error::from_reason(format!("Failed to create sampler: {}", e)))?;

    let metrics = sampler
      .get_metrics(interval)
      .map_err(|e| napi::Error::from_reason(format!("Failed to get metrics: {}", e)))?;

    let info = sampler
      .get_soc_info();

    let result = Self {
      gpu_utilization: metrics.gpu_usage.1 as f64,
      temperature: metrics.temp.gpu_temp_avg as f64,
      power_usage: metrics.gpu_power as f64,
      name: info.chip_name.clone(),
      id: info.chip_name.clone()
    };

    Ok(result)
  }
}
