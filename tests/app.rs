// Copyright (c) 2020, Bayu Aldi Yansyah <bayualdiyansyah@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use lory::prelude::*;
use tokio::runtime::Runtime;

#[test]
fn test_request_token() {
    let config = TwitterAppConfig {
        consumer_key: "",
        consumer_secret: "",
        callback_url: "http://127.0.0.1:3000/auth/callback",
        permission: TwitterAppPermission::ReadAndWrite,
    };
    let app = TwitterApp::from(config);

    // Run async code; build the tokio runtime first
    let mut runtime = Runtime::new().unwrap();
    // Run the async code
    let authorization_url = runtime.block_on(app.get_authorization_url());
    println!("{}", authorization_url);
}
