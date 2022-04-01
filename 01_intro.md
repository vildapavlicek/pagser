# Project introduction
In the upcoming series I'd like to show you how to set up gRPC server with logging middleware, few services and logger. After that we will take a look at how we can avoid code duplication thanks to generics and how to write macros which can generate code for us so we don't have to keep writing boring stuff again and again.  
If you'd like to follow along you will need [Rust toolchain](https://www.rust-lang.org/tools/install) installed. Later on we will use [Pagila](https://github.com/devrimgunduz/pagila) database to have some data to play with.

## gRPC
> gRPC is a modern open source high performance Remote Procedure Call (RPC) framework that can run in any environment.  

`Remote Procedure Call` means that you can remotely execute function that server implemented. But how do you know what functions are implemented, what are inputs, outputs and how to call them? Definitions are handled by `protobuf` and are written in text file with `.proto` extension. Data (de)serialization and routing are handled by libraries that implement required protocols and specifications. In our case we will use gRPC library called [tonic](https://github.com/hyperium/tonic)

## Project set up
Before defining our `proto` file, we will first create project structure. In your terminal navigate to directory where you want your project to be and then execute command `cargo new pagser`. This will create basic Rust project structure for us. Now inside project's directory crate folder `protos` with file `customer_service.proto`.  
This should be your current project structure:
```bash
.
├── Cargo.toml
├── protos
│   └── customer_service.proto
└── src
    └── main.rs
```

### Definig `.proto` file
Inside `customer_service.proto` copy [this content](todo:add.link.to.file.on.git). I'll go over content of the file but if you are insterested in nitty gritty details you can check [language guide](https://developers.google.com/protocol-buffers/docs/proto3).  
We are defining one service with 3 remote procedure calls:
```grpc
service CustomerService {
  rpc SelectCustomers(CustomersRequest) returns (stream Customer);
  rpc SelectNewestCustomers(CustomersRequest) returns (NewestCustomersResponse);
  rpc CustomerDetails(CustomerDetailsRequest) returns (CustomerDetailsResponse);
}
```
You can see that the description is pretty similar to function signature in source codes. To ilustrate this further later on the `rpc SelectCustomers(CustomersRequest) returns (stream Customer);` will be generated into function signature `async fn select_customers(request: Request<CustomersRequest>) -> Result<Response<Self::SelectCustomersStream>, tonic::Status>`.  
Inputs and output are defined as `message` objects, for example:
```grpc
message CustomerDetailsRequest {
  int64 id = 1;
}
```
This will then be generated as
```rust
pub struct CustomerDetailsRequest {
    pub id: i64,
}
```

I find it good practice to name `message`s after the endpoint and whether they are input (Request) or output (Response). For example `message`'s name `CustomerDetailsRequest` makes it clear that it is input for rpc `CustomerDetails`. But that's not always the case as `SelectCustomers` returns `Customer`. But it is a streaming service. Our actual return type will be `Response<ReceiverStream>` and `Customer` will be sent via stream.

Each of these rpcs is set up so they return different type of response:
1. `SelectCustomers` returns stream
2. `SelectNewestCustomers` returns vector of `message`s (`NewestCustomersResponse` contains `repeated` field, which translates to `Vec`)
3. `CustomerDetails` returns single `message`  
  
gRPC also supports bidirectional streaming whose signature would be `rpc BiStreamingRpc (stream BiStreamingRequest) returns (stream BiStreamingResponse)`.

# Closing words
That's all for this introductory chapter. In the next one we will make custom build script to generate Rust code from our `customer_service.proto` file which will contain our structs and traits that we will use to create service that can be registered with gRPC server.