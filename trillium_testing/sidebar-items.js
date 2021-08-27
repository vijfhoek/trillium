initSidebarItems({"enum":[["Method","HTTP request methods."],["Status","HTTP response status codes."]],"fn":[["block_on","Runs the global and the local executor on the current thread"],["init","initialize a handler"],["with_server","Starts a trillium handler bound to a random available port on localhost, run the async tests provided as the second argument, and then shut down the server. useful for full integration tests that actually exercise the tcp layer."]],"macro":[["assert_body","assert that the response body is as specified. this assertion requires mutation of the conn."],["assert_body_contains","asserts that the response body matches the specified pattern, using [`str::contains`]"],["assert_headers","asserts any number of response headers"],["assert_not_handled","assert that all of the following are true:"],["assert_ok","assert_ok is like [`assert_response!`] except it always asserts a status of 200 Ok."],["assert_response","combines several other assertions. this assertion can be used to assert:"],["assert_status","assert that the status code of a conn is as specified."]],"mod":[["methods","`TestConn` builders for http methods"],["prelude","useful stuff for testing trillium apps"]],"struct":[["TestConn","A wrapper around a [`trillium::Conn`] for testing"],["TestTransport","a readable and writable transport for testing"],["Url","A parsed URL record."]],"trait":[["AsyncRead","Read bytes asynchronously."],["AsyncReadExt","Extension trait for [`AsyncRead`]."],["AsyncWrite","Write bytes asynchronously."]]});