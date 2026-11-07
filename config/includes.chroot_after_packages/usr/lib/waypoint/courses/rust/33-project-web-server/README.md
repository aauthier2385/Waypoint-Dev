# Lesson 33: Project: a web server

**What you'll learn**
- How the web actually works, underneath every website you visit
- What a request and a response look like, as plain text
- Accepting network connections with `std::net`
- Handling many visitors at once with threads

This is the last lesson. You'll build a real web server that your browser can visit, using only Rust's standard library.

## The idea: the web is just text

When you visit a website, your browser opens a connection to another computer, the **server**, and sends a short message, the **request**. The server sends back a message, the **response**. That's the whole web. Both messages follow a format called **HTTP**, *HyperText Transfer Protocol*, and they're just text.

Here's a real request. It's what your browser sends when you visit `http://localhost:7878/hello?name=Sam`:

```
GET /hello?name=Sam HTTP/1.1
Host: localhost:7878
User-Agent: Firefox
```

The first line is the important one:

- **`GET`** is the **method**. `GET` means "please give me this page." Sending a form uses `POST`.
- **`/hello`** is the **path**, which page is wanted.
- **`?name=Sam`** is the **query**, extra information as `key=value` pairs joined by `&`.
- **`HTTP/1.1`** is the version of HTTP being used.

And here's a response:

```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 11
Connection: close

Hello, Sam!
```

- **`200 OK`** is the **status**. `200` means success, `404` means "not found," and `405` means "you can't do that here."
- The next lines are **headers**: facts about the response. `Content-Type` says what kind of content it is, like `text/html` for a web page or `text/plain` for plain text. `Content-Length` says how many bytes the body is.
- **A blank line** separates the headers from the **body**, which is the content itself.
- Every line ends with `\r\n`, a "carriage return" and a "new line." That's a leftover from typewriters, and HTTP requires it.

## Ports and `localhost`

`localhost` means "this computer." A **port** is like an apartment number for programs: one computer can run many servers, each listening on its own port. This server uses port 7878. Websites normally use 80 or 443.

## How this server works

`src/main.rs` opens a **listener** on a port and hands it to `serve` in `src/lib.rs`:

```rust
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
serve(listener);
```

`serve`, which is finished, waits for visitors. For each connection, it starts a thread (Lesson 26), so many visitors can be handled at once, and calls `handle_connection`, which:

1. Reads the request's first line.
2. Calls **your** `parse_request_line` to understand it.
3. Calls **your** `route` to decide what to send back.
4. Calls **your** `to_http` to turn the response into text, and sends it.

## Your challenge

Open `src/lib.rs` and replace each `todo!()`:

1. **`parse_request_line`**: turn `"GET /hello?name=Sam HTTP/1.1"` into a `Request` with method `"GET"`, path `"/hello"`, and query `Some("name=Sam")`. A request without `?` has no query. Give back `None` if there aren't three parts, if the third doesn't start with `HTTP/`, or if the path doesn't start with `/`.
2. **`query_value`**: in `"name=Sam&color=blue"`, the value for `"color"` is `Some("blue")`.
3. **`route`**:
   - Any method other than `GET`: status `405`, `text/plain`, `"Only GET is allowed."`
   - `/`: status `200`, `text/html`, `"<h1>Welcome to my Rust server!</h1>"`
   - `/hello`: status `200`, `text/plain`, `"Hello, Sam!"`, using the `name` from the query, or `"stranger"` if there isn't one.
   - Anything else: status `404`, `text/plain`, `"Nothing here."`
4. **`to_http`**: turn a `Response` into text in exactly the format shown above, including `Connection: close`.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`. The last check starts your server for real and visits it over the network.

## Then visit it

```
cd ../33-project-web-server
cargo run
```

Open Firefox and go to <http://localhost:7878>, then <http://localhost:7878/hello?name=YourName>. Press **Ctrl+C** in the terminal to stop the server.

## Stuck?

- **`parse_request_line`:** `let mut parts = line.split_whitespace();`, then `let method = parts.next()?;` three times. Inside a function that gives back `Option`, `?` works for `Option` too.
- **The query:** `match target.split_once('?') { Some((path, query)) => ..., None => ... }`.
- **`query_value`:** loop over `query.split('&')`, and `split_once('=')` each pair.
- **`to_http`:** one `format!` with `\r\n` at the end of each line, and `\r\n\r\n` before the body. `self.body.len()` is the length.

## Where to go from here

You've finished the whole course! You can write Rust programs from scratch, organize them into crates and workspaces, handle errors, test your code, use threads and async, and talk to the network.

Real web servers use crates like **axum** or **actix-web**, which handle the full HTTP format, security, and thousands of connections with async. They work exactly like what you built, with many more features. Some ideas for what's next:

- Add a page that shows the current time, or counts how many visitors have come.
- Rebuild this server with axum: `cargo new`, then `cargo add axum tokio --features tokio/full`.
- Read **The Rust Programming Language**, the official free book, at doc.rust-lang.org/book.
- Practice with **Rustlings**, small exercises like these lessons, at github.com/rust-lang/rustlings.
- Build something you'll actually use. That's how programmers really learn.

## New words

- **HTTP**: the text format browsers and servers use to talk.
- **Request / response**: the message the browser sends, and the one the server sends back.
- **Status code**: a number saying how a request went, like 200 or 404.
- **Header**: a line of information about a request or response.
- **Port**: a number that tells a computer which program a connection is for.
