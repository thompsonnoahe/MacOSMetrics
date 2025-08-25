use macos_metrics::metrics::Sampler;

#[test]
fn get_gpu_usage() {
  let mut sampler = Sampler::new().unwrap();

  let metrics = sampler.get_metrics(1000).unwrap();

  let info = sampler.get_soc_info();

  println!("{:?}", metrics);

  println!("{:?}", info);
}
