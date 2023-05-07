use std::{net::IpAddr, time::Duration, collections::HashMap};
use futures::future::join_all;
use lazy_static::lazy_static;
use rand::random;
use surge_ping::{Client, PingIdentifier, PingSequence, IcmpPacket};
use tokio::sync::Semaphore;

use crate::{types::Status, inmemory};


lazy_static! {
    static ref SEMAPHORE_AUTO_DISCOVERY: Semaphore = Semaphore::new(1);

    static ref CACHE: HashMap<String, Status> = HashMap::new();
}


pub async fn status_check_all()  {
    let servers = inmemory::get_all_servers();

    let permit = SEMAPHORE_AUTO_DISCOVERY.acquire().await.unwrap();

    match Client::new(&surge_ping::Config::default()) {
        Ok(client) => {
            // list of async tasks executed by tokio
            let mut tasks = Vec::new();

            for server in servers {
                if server.ipaddress.trim().is_empty() {
                    continue;
                }

               
                match server.ipaddress.parse() {
                    Ok(ipaddress) => {
                        tasks.push(tokio::spawn(get_host_status(
                            IpAddr::V4(ipaddress),
                            client.clone(),
                        )));
                    }
                    Err(err) => {
                        log::error!("Error while parsing address {:?} was {:?}", server.ipaddress, err);
                    }
                }
            }

            // wait for all tasks to finish
            let task_results = join_all(tasks).await;

            let results_from_query: Vec<Status> = task_results
                .iter()
                .map(move |r| r.as_ref().unwrap().to_owned())
                .collect();
            
            log::debug!("inserting {} status into cache", results_from_query.len());

            inmemory::insert_status(results_from_query);
        }
        e => {
            log::error!("Error while creating ping client: {:?}", e.err());
        }
    };

    drop(permit);    
}

pub async fn status_check(ips_to_check: Vec<String>, use_cache: bool) -> Result<Vec<Status>, std::io::Error> {
    
    let list_to_check = if ips_to_check.is_empty() {
        inmemory::get_all_servers().iter().map(|s| s.ipaddress.clone()).collect()
    }
    else {
        ips_to_check
    };

    let result = if use_cache {
        list_to_check.iter().map(|ipaddress| inmemory::get_status(ipaddress.clone()).unwrap_or_else(|| Status::new(ipaddress.clone()))).collect()
    }
    else {
        Vec::new()
    };
    Ok(result)
}


pub async fn ping(addr: IpAddr, client: Client) -> bool {
    let payload = [0; 56];
    let mut pinger = client.pinger(addr, PingIdentifier(random())).await;
    pinger.timeout(Duration::from_secs(1));

    let mut interval = tokio::time::interval(Duration::from_secs(1));
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
            Err(_e) => {
                reachable = false;
            }
        };
    }
    reachable
}

async fn get_host_status(addr: IpAddr, client: Client) -> Status {
    let result = ping(addr, client).await;

    Status {
        ipaddress: addr.to_string(),
        is_running: result,
    }
}


