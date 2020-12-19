#[derive(Clone)]
pub struct State {}

pub mod gql_bootstrap {
  use crate::infrastructures::State;
  use tide::Server;

  pub struct Bootstrap;
  impl Bootstrap {
    pub fn create_server() -> Server<State> {
      create_graphql_server()
    }
  }
}

pub mod rest_bootstrap {
  use crate::infrastructures::State;
  use tide::Server;

  pub struct Bootstrap;
  impl Bootstrap {
    pub fn create_server() -> Server<State> {
      create_graphql_server()
    }
  }
}
