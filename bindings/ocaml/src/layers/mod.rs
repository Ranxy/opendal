use ::opendal as od;
use std::time::Duration;

#[ocaml::sig]
pub struct RetryLayer(pub od::layers::RetryLayer);
ocaml::custom!(RetryLayer);

#[ocaml::sig]
pub struct ImmutableIndexLayer(pub od::layers::ImmutableIndexLayer);
ocaml::custom!(ImmutableIndexLayer);

#[ocaml::sig]
pub struct ConcurrentLimitLayer(pub od::layers::ConcurrentLimitLayer);
ocaml::custom!(ConcurrentLimitLayer);

#[ocaml::sig]
pub struct TimeoutLayer(pub od::layers::TimeoutLayer);
ocaml::custom!(TimeoutLayer);


#[ocaml::sig]
pub struct LoggingLayer(pub od::layers::LoggingLayer);
ocaml::custom!(LoggingLayer);

#[ocaml::sig(
    "
| ConcurrentLimit of concurrent_limit_layer
| ImmutableIndex of immutable_index_layer
| Retry of retry_layer
| Timeout of timeout_layer
| Logging of logging_layer
"
)]
pub enum Layer {
    ConcurrentLimit(ConcurrentLimitLayer),
    ImmutableIndex(ImmutableIndexLayer),
    Retry(RetryLayer),
    Timeout(TimeoutLayer),
    Logging(LoggingLayer),
}
ocaml::custom!(Layer);

#[ocaml::func]
#[ocaml::sig("int option -> float option -> bool -> float option -> float option -> layer ")]
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
        retry = retry.with_max_delay(Duration::from_millis(max_delay as u64));
    }
    if let Some(min_delay) = min_delay {
        retry = retry.with_min_delay(Duration::from_millis(min_delay as u64));
    }
    Layer::Retry(RetryLayer(retry)).into()
}

#[ocaml::func]
#[ocaml::sig("string array -> layer ")]
pub fn new_immutable_index_layer(keys: Vec<String>) -> ocaml::Pointer<Layer> {
    let mut layer = ImmutableIndexLayer(od::layers::ImmutableIndexLayer::default());

    for key in keys {
        layer.0.insert(key)
    }

    Layer::ImmutableIndex(layer).into()
}

#[ocaml::func]
#[ocaml::sig("int -> layer ")]
pub fn new_concurrent_limit_layer(permits: usize) -> ocaml::Pointer<Layer> {
    Layer::ConcurrentLimit(ConcurrentLimitLayer(od::layers::ConcurrentLimitLayer::new(
        permits,
    )))
    .into()
}

#[ocaml::func]
#[ocaml::sig("float -> int64 option -> layer ")]
pub fn new_timeout_layer(timeout: f64, speed: Option<u64>) -> ocaml::Pointer<Layer> {
    let mut layer = od::layers::TimeoutLayer::default();
    layer = layer.with_timeout(Duration::from_millis(timeout as u64));
    if let Some(speed) = speed {
        layer = layer.with_speed(speed);
    }

    Layer::Timeout(TimeoutLayer(layer)).into()
}


#[ocaml::func]
#[ocaml::sig("float -> layer ")]
pub fn new_logging_layer(timeout: f64,) -> ocaml::Pointer<Layer> {
    println!("---:{}",timeout);
    let layer = od::layers::LoggingLayer::default();
    Layer::Logging(LoggingLayer(layer)).into()
}
