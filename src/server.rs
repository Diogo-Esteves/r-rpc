  use tonic::{transport::Server, Request, Response, Status};
  use helloworld::greeter_server::{Greeter, GreeterServer};
  use helloworld::{HelloRequest, HelloReply};

  pub mod helloworld {
      tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
  }

  #[derive(Default)]
  pub struct MyGreeter {}

  #[tonic::async_trait]
  impl Greeter for MyGreeter {
      async fn say_hello(
          &self,
          request: Request<HelloRequest>,
      ) -> Result<Response<HelloReply>, Status> {
          let reply = HelloReply {
              message: format!("Hello {}!", request.into_inner().name),
          };
          Ok(Response::new(reply))
      }
  }

  #[tokio::main]
  async fn main() -> Result<(), Box<dyn std::error::Error>> {
      let addr = "[::1]:50051".parse()?;
      let greeter = MyGreeter::default();

      Server::builder()
          .add_service(GreeterServer::new(greeter))
          .serve(addr)
          .await?;

      Ok(())
  }

