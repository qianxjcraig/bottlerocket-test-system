mod constants;
mod resource_provider;
mod test;

pub use constants::{API_VERSION, ENV_TEST_NAME, NAMESPACE, TESTSYS};
pub use resource_provider::{ResourceProvider, ResourceProviderSpec, ResourceProviderStatus};
pub use test::{
    AgentStatus, ControllerStatus, Lifecycle, ResourceStatus, RunState, Test, TestResults,
    TestSpec, TestStatus,
};

use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

/// The `Configuration` trait is for structs that can be used for custom data, which is represented
/// in a CRD model like this:
///
/// ```yaml
/// configuration:
///   additionalProperties: true
///   nullable: true
///   type: object
/// ```
///
/// The traits aggregated by the `Configuration` trait are typical of "plain old data" types and
/// provide a way for clients to strongly type this data which is otherwise unconstrained by the
/// API.
///
pub trait Configuration:
    Serialize + DeserializeOwned + Clone + Debug + Default + Send + Sync + Sized + 'static
{
}
