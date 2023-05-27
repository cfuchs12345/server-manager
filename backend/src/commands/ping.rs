use std::{net::IpAddr, time::Duration, any::Any};
use async_trait::async_trait;
use rand::random;
use surge_ping::{Client, PingIdentifier, PingSequence, IcmpPacket, Config};

use crate::{models::error::AppError};

use super::{Command,CommandInput, CommandResult};


pub const PING: &str = "ping";


#[derive(Clone)]
pub struct PingCommand {
}

impl PingCommand {
    pub fn new() -> Self {
        PingCommand {
        }
    }
}

#[async_trait]
impl Command for PingCommand {
    fn get_name(&self) -> &str {
        PING
    }

    async fn execute(&self, input: &CommandInput) -> Result<Box<dyn Any + Sync + Send>, AppError> {
        let client = Client::new(&Config::default())?;

        if let Some(ipaddress) =  input.get_ipaddress() {
            let payload = [0; 56];
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            

            let mut pinger = client.pinger(ipaddress, PingIdentifier(random())).await;
            pinger.timeout(Duration::from_secs(1));
        
            let mut reachable = false;

        
            for idx in 0..3 {
                interval.tick().await;
                match pinger.ping(PingSequence(idx), &payload).await {
                    Ok((IcmpPacket::V4(_packet), _dur)) => {
                        reachable = true;
                        break;
                    }
                    Ok((IcmpPacket::V6(_packet), _dur)) => {
                        reachable = true;
                        break;
                    }
                    Err(_err) => {
                        reachable = false;
                    }
                };
            }
            return Ok(Box::new(PingCommandResult::new(reachable, ipaddress)));
        } else {
            return Err(AppError::MissingArgument("ipaddress".to_string()));
        }
    }
}


pub fn make_input(address: IpAddr) -> CommandInput {
    CommandInput::new(PING, None, Some(address), Vec::new(), super::Parameters::empty(), Vec::new())
}

#[derive(Clone)]
pub struct PingCommandResult {
    result: bool,
    ipaddress: IpAddr
}
impl PingCommandResult {
    fn new(result: bool,ipaddress: IpAddr) -> Self {
        PingCommandResult {result, ipaddress}
    }

    pub fn get_ipaddress(&self) -> IpAddr {
       self.ipaddress
    }

    pub fn get_result(&self) -> bool {
        self.result
    }
}

impl CommandResult for PingCommandResult {

}
