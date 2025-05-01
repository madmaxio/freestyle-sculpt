pub mod deformation;
pub mod integrations;
pub mod meshgraph;
pub mod ray;
pub mod selectors;

#[cfg(feature = "rerun")]
pub mod utils;

#[cfg(feature = "rerun")]
lazy_static::lazy_static! {
    pub static ref RR: rerun::RecordingStream = rerun::RecordingStreamBuilder::new("freestyle_s culpt").spawn().unwrap();
}
