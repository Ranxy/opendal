use ::opendal as od;
use std::time::Duration;

#[ocaml::sig]
pub enum Layer {
    // ConcurrentLimit(ConcurrentLimitLayer),
    // ImmutableIndex(ImmutableIndexLayer),
    Retry(RetryLayer),
}
ocaml::custom!(Layer);

#[ocaml::sig]
pub struct RetryLayer(pub od::layers::RetryLayer);
ocaml::custom!(RetryLayer);

#[ocaml::func]
#[ocaml::sig("int option -> float option -> bool -> float option -> float option -> retry_layer ")]
pub fn new_retry_layer(
    max_times: Option<usize>,
    factor: Option<f32>,
    jitter: bool,
    max_delay: Option<f64>,
    min_delay: Option<f64>,
) -> ocaml::Pointer<Layer> {
    let mut retry = od::layers::RetryLayer::default();

    if let Some(max_times) = max_times {
        retry = retry.with_max_times(max_times);
    }
    if let Some(factor) = factor {
        retry = retry.with_factor(factor);
    }
    if jitter {
        retry = retry.with_jitter();
    }
    if let Some(max_delay) = max_delay {
        retry = retry.with_max_delay(Duration::from_micros((max_delay * 1000000.0) as u64));
    }
    if let Some(min_delay) = min_delay {
        retry = retry.with_min_delay(Duration::from_micros((min_delay * 1000000.0) as u64));
    }
    Layer::Retry( RetryLayer(retry)).into()
}
