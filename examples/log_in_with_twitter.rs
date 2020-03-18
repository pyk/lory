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

// This is an example code on how to implement Log in with Twitter
// using lory
//
// To run the example, run the following:
//
// cargo run --example log_in_with_twitter
//
use lory::prelude::*;
use warp::Filter;

#[tokio::main]
async fn main() {
    // TODO: initialize Twitter client here
    let config = TwitterAppConfig {
        consumer_key: "test",
        consumer_secret: "test",
    };
    let app = TwitterApp::from(config);

    // GET /
    // Render home page with 'sign in with twitter' button
    let home_page = warp::get().map(|| {
        // Generate login url
        let oauth_token = app.request_token().await;
        let login_url = format!(
            "https://api.twitter.com/oauth/authenticate?oauth_token={}",
            oauth_token
        );

        warp::reply::html(format!(
            "
            <html>
                <head>
                    <title>Login with Twitter - lory example</title>
                </head>
                <body>
                    <h1>Welcome</h1>
                    <a href='{}'>Sign in via Twitter</a>
                </body>
            </html>
            ",
            login_url
        ))
    });

    let routes = home_page;

    println!("Server is running at http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
