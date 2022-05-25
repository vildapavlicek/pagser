# Introduction
In previous two chapters we've set up project and defined and implemented our gRPC. In this chapter we will take a look on how to get server up and running.

# Setting up server
Our server code for this chapter can be found [here](https://github.com/vildapavlicek/pagser/blob/03-server/src/grpc/server.rs).

In our `src/grpc/server.rs` file create function
```rust
pub async fn run() {}
```

and then add this code inside

```rust
let addr = std::env::var("PAGSER_ADDR") // 1
        .unwrap_or_else(|_| "[::1]:50051".into()) // 2
        .parse::<SocketAddr>() // 3
        .expect("failed parse address to bind to"); // 4
```
1. First we check if there is set environment variable `PAGSER_ADDR`
2. If it is not set, we will fallback to default value of `[::1]:50051` which is IPv6 version of localhost
3. Then we try to parse the address to `SocketAddr` which is enum that can be either IPv4 or IPv6 address
4. And as last we unwrap our value, panicking if it would be error

After that we start our server as follows:
```rust 
Server::builder() // 1
        .add_service(CustomerServiceServer::new(CustomerService)) / // 2
        .serve(addr) // 3
        .await // 4
        .expect("failed to run server"); // 5
```
1. We use builder pattern to configure the server
2. We add our `CustomerService` struct which we implemented before, but first we have to wrap it in `CustomerServiceServer` which is generated based on our gRPC. What happens under the hood, simply put, is that we register service that has method `call(..)` which gets called with each request and is expected to return response. Our `call(..)` simply matches the request's URI and based on that calls the requested function implemented on our `CustomerService`. You can see implementation details in our generate source file.
3. Then we create `Future` that will listen and server all our requests
4. And `await` it
5. If there happen any errors, we will just panick and exit the aplication

# Make `main` async (again)
Now we just have to call our`run()` function from our `main` and give our server few test calls.  
But our `run()` is `async` and you can only call `async` functions only from other `async` functions, which our main isn't. Luckily for us the only thing we have to do is update signature of our `main` from 
```rust
fn main() {
    println!("Hello, world!");
}
```
to
```rust
#[tokio::main] // 1
async fn main() -> Result<(), Box<dyn std::error::Error>> { // 2
    grpc::server::run().await; // 3
    Ok(()) // 4
}
```
1. `tokio` (our async library and runtime) gives us this nice macro which generates code that sets up runtime for us and calls our `main`. Alternative would be to create runtime manually with `tokio::runtime::Builder` and then call `block_on(grpc::server::run())` but unless you need some specific configuration, it is best to use macro
2. Our `main`'s signature gets changed to this as this is what is expected by the macro
3. Run our server
4. When `run()` finishes, return with `Ok(())`

# Closing words
Now if you start application with `cargo r` and use some of the clients to make test call ([BloomRPC](https://github.com/bloomrpc/bloomrpc), [Postman](https://www.postman.com/), yes since 2022 Postman also supports gRPC, but it is still in beta) you should get back error response:
```
{
  "error": "12 UNIMPLEMENTED: Not yet implemented"
}
```
In next chapter we will take a look at logging, errors and logging middleware.