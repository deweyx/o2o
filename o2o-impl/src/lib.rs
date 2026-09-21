#[cfg(any( all(feature = "syn", any(feature = "syn2", feature = "syn3")), all(feature = "syn2", feature = "syn3") ))]
compile_error!("Features 'syn' and 'syn2' cannot be enabled at the same time");

#[cfg(not(any( all(feature = "syn", any(feature = "syn2", feature = "syn3")), all(feature = "syn2", feature = "syn3") )))]
mod ast;
#[cfg(not(any( all(feature = "syn", any(feature = "syn2", feature = "syn3")), all(feature = "syn2", feature = "syn3") )))]
mod attr;
#[cfg(not(any( all(feature = "syn", any(feature = "syn2", feature = "syn3")), all(feature = "syn2", feature = "syn3") )))]
pub mod expand;
#[cfg(not(any( all(feature = "syn", any(feature = "syn2", feature = "syn3")), all(feature = "syn2", feature = "syn3") )))]
mod kw;
#[cfg(not(any( all(feature = "syn", any(feature = "syn2", feature = "syn3")), all(feature = "syn2", feature = "syn3") )))]
mod validate;

mod tests;