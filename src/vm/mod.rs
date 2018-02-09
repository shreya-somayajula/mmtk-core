mod object_model;
mod scanning;
mod collection;
mod active_plan;
pub mod unboxed_size_constants;
pub use self::object_model::ObjectModel;
pub use self::scanning::Scanning;
pub use self::collection::Collection;
pub use self::active_plan::ActivePlan;

#[cfg(feature = "jikesrvm")]
pub mod jikesrvm;

#[cfg(feature = "jikesrvm")]
pub use self::jikesrvm::object_model::VMObjectModel as VMObjectModel;

#[cfg(feature = "jikesrvm")]
pub use self::jikesrvm::scanning::VMScanning as VMScanning;

#[cfg(feature = "jikesrvm")]
pub use self::jikesrvm::collection::VMCollection as VMCollection;

#[cfg(feature = "jikesrvm")]
pub use self::jikesrvm::active_plan::VMActivePlan as VMActivePlan;

#[cfg(feature = "jikesrvm")]
pub use self::jikesrvm::JikesRVM;

#[cfg(not(feature = "jikesrvm"))]
mod openjdk;

#[cfg(not(feature = "jikesrvm"))]
pub use self::openjdk::*;

#[cfg(not(feature = "jikesrvm"))]
pub use self::openjdk::object_model::VMObjectModel as VMObjectModel;

#[cfg(not(feature = "jikesrvm"))]
pub use self::openjdk::scanning::VMScanning as VMScanning;

#[cfg(not(feature = "jikesrvm"))]
pub use self::openjdk::collection::VMCollection as VMCollection;

#[cfg(not(feature = "jikesrvm"))]
pub use self::openjdk::active_plan::VMActivePlan as VMActivePlan;