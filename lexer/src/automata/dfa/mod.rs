mod automata;
mod visualizer;
mod builder;
mod optimize;

pub use automata::DFAutomata;
pub use visualizer::DFAVisualizer;
pub(super) use optimize::DFAOptimizer;