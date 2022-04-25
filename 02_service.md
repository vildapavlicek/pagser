# Introduction
In this chapter we will add custom build script to compile `customer_service.proto` file, add server to our source and implement the service albeit returning only error for now.

## Dependencies
Since from now on we will depend on other libraries, let's bring in all the dependencies we will need in the future. Just copy all the dependencies from [Cargo.toml](link-to-full-dependency-file) to your `Cargo.toml`

# Customizing build script
> Placing a file named build.rs in the root of a package will cause Cargo to compile that script and execute it just before building the package.  
  
This is taken from [official documentation](https://doc.rust-lang.org/cargo/reference/build-scripts.html) and is exactly what we need. We need to first generate Rust code based on definitions in our `customer_service.proto` so we can then use it in our application.

## Compiling `customer_service.proto`
First let's create `build.rs` file and then copy there contents of [build.rs](point-to-build-rs-file). At the moment our `build.rs` does two things:  
1. compile `.proto` file  
2. [export commit hash as compile-time environment variable](https://stackoverflow.com/questions/43753491/include-git-commit-hash-as-string-into-rust-program)

Explaining proto file compilation:
```Rust
tonic_build::configure()
        .build_client(false)
        .compile(&["protos/customer_service.proto"], &["protos/"])?;
```
`tonic_build` is part of the [tonic](https://github.com/hyperium/tonic) framework and is used to compile `.proto` files. Calling `configure()` returns a builder that allows us to configure the compilation process of our `.proto` files. Because we don't need client related code, we set `.build_client(false)` and then pass list of our files we wish to compile to `.compile()` and that's all.  
  
In `Cargo.toml` you can notice that `tonic-build` is under `[build-dependencies]`. These dependencies are used only during build process and are not part of the final binary.  
Now if you run `cargo build` in terminal in your project's directory you should find the result of our compiled `.proto` file in `/target/debug/build/pagser-{ID}/out/customer_service.rs`. There will be multiple `pagser-{ID}` folders, so you will have check for the correct one.

In generated file you will find structs and traits (interfaces in Rust) that has to be implemented to be able to register our service with server.

# Customer service
## Preparations
First create new module where we will store our code. Update your `src` folder to look like this:
```
./src
├── grpc <-- new folder
│   ├── customer_service.rs <-- new file
│   ├── mod.rs <-- new file
│   └── server.rs <-- new file
└── main.rs
```
In `customer_service.rs` we will implement our gRPC service and in `server.rs` we will set up server later on.
Now we need to bring into scope our generated code. For that we can use tonic's macro `tonic::include_proto()!` and so into `grpc/mod.rs` copy [this](link/to/github).

## Implementing customer service
For now we will always return error - Not Implemented. Implementation of our `customer_service.rs` can be found [here](link/to/git).   
First we need some kind of object that we will implement required trait for. For now we can use just empty (zero-sized) struct:
```Rust
pub struct CustomerService;
```
and after that implement `CustomerService` trait: 
```Rust
#[tonic::async_trait] // 1.
impl customer_service_server::CustomerService for CustomerService {
        ...
        async fn customer_details( //2.
                &self, // 3.
                request: Request<CustomerDetailsRequest>, // 4.
        ) -> Result<Response<CustomerDetailsResponse>, Status> // 5.
        {
                Err(Status::unimplemented("Not yet implemented".to_string())) // 6.
        }
}
```
Let's explain a bit what is going on here 
1. At the time of writing Rust doesn't officially support async traits so to work around that we use macro `async_trait` which lets use write async traits without any issues.  
2. that is our rpc we've defined in `.proto` file
3. referencing object we've implemented the `CustomerService` trait for, in our case it's `CustomerService` struct, we will need access to it later to be able to work with database
4. except the `&self` we get only one argument and that is `Request` which wraps around struct that is the gRPC message, again as defined in our `.proto` file
5. return type is of `Result` that in case we successfuly finish our computation returns `Ok` variant which wraps `Response` containing gRPC message, in case of error we return `Err` variant with `Status`
6. for now we will always return `Err` variant with `Status` of `unimplemented` and our own message (in this case it is of type String)

All the other implementation are the same and we will go into more details later on when we actually implement them.

# Closing words
That's all for this chapter. In the next one we will set up server, register our service and see if it works.