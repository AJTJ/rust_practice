file:///Users/ajtj/.rustup/toolchains/stable-x86_64-apple-darwin/share/doc/rust/html/book/ch20-01-single-threaded.html#validating-the-request-and-selectively-responding

## Notes
- HTTP Protocol
- CRLF = "carriage return and line feed"
  - i.e. new line
- GET requests have no body


```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body

Request: GET /smack HTTP/1.1
Host: 127.0.0.1:7878
Connection: keep-alive
Cache-Control: max-age=0
sec-ch-ua: "Google Chrome";v="87", " Not;A Brand";v="99", "Chromium";v="87"
sec-ch-ua-mobile: ?0
Upgrade-Insecure-Requests:
...
...
```