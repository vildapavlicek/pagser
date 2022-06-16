# Introduction
In this chapter we will add logging and then add logging layer to our gRPC server. As both topics are pretty complex and each would deserve its own post(s) I won't be going into much details here.

## Setting up logger
Before we can do any kind of logging, we have to set up and configure our logging library. We will use set of crates centered around [tracing](https://crates.io/crates/tracing) which defines itself as `a framework for instrumenting Rust programs to collect structured, event-based diagnostic information`. We will log both to the console and file at the same time.

### Logger implementation
First we will create `src/logger.rs` file where we will write our [code]().

```rust
pub fn init() -> WorkerGuard {}
```
We will create our function `init` which will take 0 inputs and return guard. If we want to log to file, someone has to "own" that file so we can write into it. This is what our guard is for. If guard is dropped, we loose all access to the log file and thus stop writing logs in it.

```rust
    // 1
    let file_appender = tracing_appender::rolling::daily(".", env!("CARGO_PKG_NAME"));
    // 2
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
}
```
1. we create a rolling appender, that will create new file each day, file will be created at current folder (`.`) and named after our project with a date suffix, so the resulting name will be for example `pagser.2022-06-16`. `CARGO_PKG_NAME` is environment variable available during compilation time and `env!()` macro `expands to the value of the named environment variable at compile time`.
2. pass our appender to helper function which returns `writer` and guard

Now we have file appender and guard. Appender will append logs to file as well a rotating it while guard makes sure that file is not closed.

```rust
    tracing_subscriber::Registry::default() // 1
        .with(EnvFilter::new(format!( //2
            "{}=trace",
            env!("CARGO_PKG_NAME")
        )))
        .with(
            tracing_subscriber::fmt::layer() // 3
                .with_writer(file_writer) // 3
                .with_ansi(false),  // 4
        )        
```
1. we create a `Registry` which, simply put, stores logs related data and exposes them to `layers` which in our case is file writer and later on stdout (console) writer
2.  specific format of setting what to log and at what level, for more information see [EnvFilter](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/struct.EnvFilter.html)
3. we create layer and give it our file appender as a writer, so it will use it to write logs to file
4. disable ANSI terminal colors, we don't want them in file

Next we add another layer that will write our logs to console.
```rust
.with(tracing_subscriber::fmt::layer().with_ansi(true).pretty())
```
It's almost the same as above, but we do not pass it any writer and let it resolve to stdout on its own. This time we set `with_ansi()` to `true` so the console output is colored and also tell it to use nice, readable formatting.

To finish it off we call `.init()` which tries to initialize logger with our configuration and layers. If any logger was initialized before, this call will panic.

Now update our main to initialize our logger:
```rust
let _guard = logger::init();
    info!(
        version = env!("CARGO_PKG_VERSION"),
        git_hash = env!("GIT_HASH"), // remove if you don't have git
        "Starting server..."
    );
```
No if you run our server you should see in console:
```bash
2022-06-16T12:00:24.029761Z  INFO pagser: Starting server..., version: "0.1.0", git_hash: "fc5a3b2ce2366e430ec722456301383d42a574c5"
    at src\main.rs:9
```
`let _guard = init()` is important, as it will make sure our guard is released when returning from main. If you wouldn't do the assigment, guard would be dropped right away and with it logger's access to file.

## Add logging layer to server
In Rust most servers are build using more primitive libraries like [`http`](https://docs.rs/http/latest/http/) and [`tower`](https://docs.rs/tower/latest/tower/). This ensures that you can easily interface with them at different levels. For us that means, that if we want to add logging middleware to our server, we have to create `tower` layer and pass it to server builder. Thanks to [`tower_http`](https://docs.rs/tower-http/latest/tower_http/) crate it is extremelly easy - all we have to do is create closures that will be called on specific events.

First we create tracing layer specific for gRPC:
```rust
tower_http::trace::TraceLayer::new_for_grpc()
```
after that we will create a special span, that will add Uuid to each log message for given request
```rust
.make_span_with(|_request: &http::Request<Body>| {
                tracing::info_span!("grpc_request", uuid = %uuid::Uuid::new_v4().to_string())
            })
```
then we tell it what to do when we receive request:
```rust
.on_request(|request: &http::Request<Body>, _span: &Span| info!(path = %request.uri().path(), "received new request"))
```
In this case, wil just log message, that we received request and the path where it was sent to.
Next we add logging on response and on failure:
```rust
.on_response(|_response: &http::Response<BoxBody>, latency: Duration, _span: &Span| {
    debug!(?latency, "generated response")
})
.on_failure(|failure_class: GrpcFailureClass, latency: Duration, _span: &Span| {
    error!(?latency, ?failure_class, "generated response with error code")
})
```
For each response we will just log how long it took and message that we created response. If we return response with status code, we will also log error code.

If you run our server and send request, this is what you should see in console (and also in file):
```bash
2022-06-16T12:40:46.776043Z  INFO pagser::grpc::server: received new request, path: /customer_service.CustomerService/SelectCustomers
    at src\grpc\server.rs:21
    in pagser::grpc::server::grpc_request with uuid: 5f356ef2-64c4-413a-8cab-fa961943076b

  2022-06-16T12:40:46.780614Z DEBUG pagser::grpc::server: generated response, latency: 3.988ms
    at src\grpc\server.rs:23
    in pagser::grpc::server::grpc_request with uuid: 5f356ef2-64c4-413a-8cab-fa961943076b

  2022-06-16T12:40:46.785296Z ERROR pagser::grpc::server: generated response with error code, latency: 3.988ms, failure_class: Code(12)
    at src\grpc\server.rs:26
    in pagser::grpc::server::grpc_request with uuid: 5f356ef2-64c4-413a-8cab-fa961943076b
```
You can see we have logged 3 messages - on recieving request, on generating response and then because the response was error, we also triggered on failure event.


# Closing words
There is a lot going on in relation to logging and then adding services / layers to our server and if you are more interested in it I'd advise you to visit documentation pages of given crates where it is described in much more details. But now you should atleast have rough idea about what is going on and how to use it.