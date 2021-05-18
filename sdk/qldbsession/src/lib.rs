#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
/// <p>The transactional data APIs for Amazon QLDB</p>
/// <note>
/// <p>Instead of interacting directly with this API, we recommend using the QLDB driver
/// or the QLDB shell to execute data transactions on a ledger.</p>
/// <ul>
/// <li>
/// <p>If you are working with an AWS SDK, use the QLDB driver. The driver provides
/// a high-level abstraction layer above this <i>QLDB Session</i> data
/// plane and manages <code>SendCommand</code> API calls for you. For information and
/// a list of supported programming languages, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/getting-started-driver.html">Getting started
/// with the driver</a> in the <i>Amazon QLDB Developer
/// Guide</i>.</p>
/// </li>
/// <li>
/// <p>If you are working with the AWS Command Line Interface (AWS CLI), use the
/// QLDB shell. The shell is a command line interface that uses the QLDB driver to
/// interact with a ledger. For information, see <a href="https://docs.aws.amazon.com/qldb/latest/developerguide/data-shell.html">Accessing Amazon QLDB using the
/// QLDB shell</a>.</p>
/// </li>
/// </ul>
/// </note>
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_json_errors;
mod blob_serde;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
mod json_deser;
pub mod model;
pub mod operation;
mod operation_ser;
pub mod output;
mod serde_util;
mod serializer;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("qldbsession", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;