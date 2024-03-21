use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Common error definition that is used by all procedures.
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: Cow<'static, str>,
    // XX: Maybe add some error kind in the future?
    //     See: <https://grpc.github.io/grpc/core/md_doc_statuscodes.html>
}
