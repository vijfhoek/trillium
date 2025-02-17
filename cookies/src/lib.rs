#![forbid(unsafe_code)]
#![deny(
    clippy::dbg_macro,
    missing_copy_implementations,
    rustdoc::missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    unused_qualifications
)]

/*!

# the trillium cookie handler

## example
```
use trillium::Conn;
use trillium_cookies::{cookie::Cookie, CookiesConnExt, CookiesHandler};
async fn handler_that_uses_cookies(conn: Conn) -> Conn {
    let content = if let Some(cookie_value) = conn.cookies().get("some_cookie") {
        format!("current cookie value: {}", cookie_value.value())
    } else {
        String::from("no cookie value set")
    };

    let cookie = Cookie::build("some_cookie", "some-cookie-value").path("/").finish();
    conn.with_cookie(cookie).ok(content)
}

let handler = (CookiesHandler::new(), handler_that_uses_cookies);

use trillium_testing::prelude::*;

assert_ok!(
    get("/").on(&handler),
    "no cookie value set",
    "set-cookie" => "some_cookie=some-cookie-value; Path=/"
);

assert_ok!(
    get("/").with_request_header("cookie", "some_cookie=trillium").on(&handler),
    "current cookie value: trillium",
    "set-cookie" => "some_cookie=some-cookie-value; Path=/"
);

```
*/
mod cookies_handler;
pub use cookies_handler::CookiesHandler;

mod cookies_conn_ext;
pub use cookies_conn_ext::CookiesConnExt;

pub use cookie;
