//! # dynamical-systems
//!
//! Dynamical systems in Rust — bifurcation analysis, phase portraits, stability, attractors, ODE integration.
//!
//! Covers continuous (ODE) and discrete (iterated map) systems, fixed point
//! stability via Jacobian eigenvalue analysis, bifurcation diagrams, limit
//! cycles, strange attractors, Lyapunov exponents, Poincaré maps, and basin
//! of attraction computation. Includes behavioral dynamics modeling.

pub mod continuous;
pub mod discrete;
pub mod fixed_points;
pub mod bifurcation;
pub mod limit_cycles;
pub mod strange_attractors;
pub mod poincare;
pub mod basin;
pub mod behavior;

pub use continuous::*;
pub use discrete::*;
pub use fixed_points::*;
pub use bifurcation::*;
pub use limit_cycles::*;
pub use poincare::*;
pub use strange_attractors::*;
pub use basin::*;
pub use behavior::*;
