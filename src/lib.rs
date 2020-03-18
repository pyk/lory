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

//! lory is an easy-to-use Rust library for accessing the [Twitter API].
//!
//! [Twitter API]: https://developer.twitter.com/en/docs
//!
//! # Usage
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! lory = "*"
//! ```
//!
//! and this to your crate root:
//!
//! ```
//! use lory::prelude::*;
//! ```
//!
//! To get started using lory, read the quickstart tutorial below.
//!
//! # Quickstart Tutorial
//!
//! You will need valid Twitter developer credentials in the form of a set of
//! consumer and access tokens/keys. You can get these in your [twitter apps].
//!
//! [twitter apps]: https://developer.twitter.com/en/apps
//!
//! Setup the authentication object first:
//!
//! ```
//! # use lory::prelude::*;
//! // For OAuth 1.0a, user based authentication
//! let user_auth = UserAuthentication {
//!     consumer_key: "",
//!     consumer_scret: "",
//!     access_token: "",
//!     access_secret: ""
//! }
//!
//! // For OAuth 2.0, application-only based authentication
//! let app_auth = AppAuthentication {
//!     consumer_key: "",
//!     consumer_secret: "",
//!     bearer_token: "",
//! }
//! ```
//!
//! Add your credentials accordingly.
//!
//! Then initialize new twitter client using the following code:
//!
//! ```
//! # use lory::prelude::*;
//! // For user based auth
//! let user = TwitterUser::from(user_auth);
//!
//! // For app only based auth
//! let app = TwitterApp::from(app_auth);
//! ```
//!
//! Note that Application-only auth will not allow you to perform requests to
//! API endpoints requiring a user context, such as posting tweets. However,
//! the endpoints available can have a higher rate limit.
//!
//! See the difference between user based and application-only based
//! application [here](https://developer.twitter.com/en/docs/basics/authentication/overview).
//!
//! Example, lets get [list of favorites] from the authenticated user:
//!
//! [list of favorites]: https://developer.twitter.com/en/docs/tweets/post-and-engage/api-reference/get-favorites-list
//!
//! ```
//! # use lory::prelude::*;
//! let favorites = user.favorites();
//! println!("{}", favorites);
//! ```
//!
//! Post some updates:
//!
//! ```
//! # use lory::prelude::*;
//! user.update_status("I hope COVID-19 is gone!");
//! ```
//!
//! Get most recent updates from the specified user timeline:
//!
//! ```
//! # use lory::prelude::*;
//! let public_tweets = app.home_timeline("@bayualsyah");
//! println!("{}", public_tweets);
//! ```
//!
//! That's it.
//!
//! See also: [TwitterUser], [TwitterApp].
//!
//! ## Getting help
//! Feel free to start discussion at [GitHub issues].
//!
//! [Github issues]: https://github.com/pyk/lory/issues/new/choose
//!
//! ## License
//! lory is licensed under the [Apache-2.0] license.
//!
//! Unless you explicitly state otherwise, any contribution intentionally
//! submitted for inclusion in lory by you, as defined in the Apache-2.0
//! license, shall be licensed as above, without
//! any additional terms or conditions.
//!
//! [Apache-2.0]: https://github.com/pyk/lory/blob/master/LICENSE
//!

pub mod app;
pub mod prelude;
