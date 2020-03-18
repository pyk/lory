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
use reqwest;
use std::fmt;

/// App permissions describe the access level for application-user
/// authentication. App permissions are configured per application within your
/// [Twitter app](https://developer.twitter.com/en/apps) settings.
///
/// Read more about [App permissions](https://developer.twitter.com/en/docs/basics/apps/guides/app-permissions).
pub enum TwitterAppPermission {
    /// This permission level permits read access to Twitter resources,
    /// including (for example) a user’s Tweets, home timeline, and profile
    /// information.
    ///
    /// It does not allow access to read a user’s Direct Messages, and it
    /// does not allow to update any element or object.
    ReadOnly,

    /// This permission level permits read and write access to Twitter
    /// resources. In addition to allowing read access, it also allow to post
    /// Tweets, follow users, or update elements of a user’s profile
    /// information.
    ///
    /// It also allow to hide replies on behalf of the authenticating user.
    /// This permission level does not allow any access to Direct Messages
    /// (including read, write, or delete).
    ReadAndWrite,

    /// This permission level includes access to all of the above and adds the
    /// ability to read, write and delete Direct Messages on behalf of a user.
    ReadWriteAndDM,
}

impl fmt::Display for TwitterAppPermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // the string representation is based on `x_auth_access_type`
        // params.
        // https://developer.twitter.com/en/docs/basics/authentication/api-reference/request_token
        let string = match self {
            TwitterAppPermission::ReadOnly => "read",
            TwitterAppPermission::ReadAndWrite => "write",
            TwitterAppPermission::ReadWriteAndDM => "write",
        };
        write!(f, "{}", string)
    }
}

pub struct TwitterAppConfig {
    pub consumer_key: &'static str,
    pub consumer_secret: &'static str,

    /// For OAuth 1.0a compliance this parameter is required . The value you
    /// specify here will be used as the URL a user is redirected to should
    /// they approve your application's access to their account.
    ///
    /// Set this to `oob` for out-of-band pin mode. This is also how you specify
    /// custom callbacks for use in desktop/mobile applications.
    ///
    /// Twitter require that any callback URL used with this endpoint will
    /// have to be whitelisted within the app settings on developer.twitter.com*
    pub callback_url: &'static str,

    /// Overrides the access level an application requests to a users account.
    /// This parameter is intended to allow a developer to register a read/write
    /// application but also request read only access when appropriate.
    pub permission: TwitterAppPermission,
}

pub struct TwitterApp {
    config: TwitterAppConfig,
    http_client: reqwest::Client,
    oauth_client: 
}

impl From<TwitterAppConfig> for TwitterApp {
    fn from(config: TwitterAppConfig) -> Self {
        let http_client = reqwest::Client::new();
        TwitterApp {
            config,
            http_client,
        }
    }
}

impl TwitterApp {
    /// Get the authorization URL to redirect the user.
    ///
    /// https://developer.twitter.com/en/docs/basics/authentication/api-reference/request_token
    pub async fn get_authorization_url(&self) -> &str {
        // Perform request token
        let resource_url = format!(
            "https://api.twitter.com/oauth/request_token?x_auth_access_type={}&oauth_callback={}",
            self.config.permission, self.config.callback_url
        );
        let res = self.http_client.post(&resource_url).send().await;
        println!("DEBUG res: {:?}", res);

        // let authorization_url = format!(
        //     "https://api.twitter.com/oauth/authenticate?oauth_token={}",
        //     oauth_token
        // );
        self.config.consumer_key
    }
}
