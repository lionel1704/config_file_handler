// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

use serde_json::Error as JsonError;
use std::env::VarError;
use std::io::Error as IoError;

quick_error! {
    /// Error types.
    #[derive(Debug)]
    pub enum Error {
        /// Wrapper for a `::std::env::VarError`
        Env(err: VarError) {
            description("Environment error")
            display("Environment error: {}", err)
            cause(err)
            from()
        }
        /// Wrapper for a `::std::io::Error`
        Io(err: IoError) {
            description("IO error")
            display("IO error: {}", err)
            cause(err)
            from()
        }
        /// Wrapper for a `::serde_json::Error`
        JsonParser(err: JsonError) {
            description("Json parse error")
            display("Json parse error: {}", err)
            cause(err)
            from()
        }
    }
}
