use thiserror::Error;


#[derive(Debug,Error)]
pub enum AgentErrors {
    #[error("unable to connect to the mitriko-server")]
    UnableToConnectErr,
    
}