use std::{net::IpAddr, time::Duration, collections::HashMap, io::ErrorKind};
use futures::future::join_all;
use lazy_static::lazy_static;
use log::info;
use rand::random;
use surge_ping::{Client, PingIdentifier, PingSequence, IcmpPacket};
use tokio::sync::Semaphore;

use crate::types::Status;


lazy_static! {
    static ref SEMAPHORE_AUTO_DISCOVERY: Semaphore = Semaphore::new(1);

    static ref CACHE: HashMap<String, Status> = HashMap::new();
}




pub async fn status_check(ips_to_check: Vec<String>, use_cache: bool) -> Result<Vec<Status>, std::io::Error> {
    let permit = SEMAPHORE_AUTO_DISCOVERY.acquire().await.unwrap();

    let list = match Client::new(&surge_ping::Config::default()) {
        Ok(client) => {
            // list of async tasks executed by tokio
            let mut tasks = Vec::new();
            let mut results_from_cache: Vec<Status> = Vec::new();

            for ip in ips_to_check {
                if ip.trim().is_empty() {
                    continue;
                }

                if use_cache {
                    let from_cache_res = CACHE.get(&ip);
                    if let Some(status) = from_cache_res {
                        results_from_cache.push(status.clone());
                        continue;
                    };
                }
                match ip.parse() {
                    Ok(ipaddress) => {
                        tasks.push(tokio::spawn(get_host_status(
                            IpAddr::V4(ipaddress),
                            client.clone(),
                        )));
                    }
                    Err(err) => {
                        log::error!("Error while parsing address {:?} was {:?}", ip, err);
                    }
                }
            }

            // wait for all tasks to finish
            let task_results = join_all(tasks).await;

            let mut results_from_query: Vec<Status> = task_results
                .iter()
                .map(move |r| r.as_ref().unwrap().to_owned())
                .collect();
            results_from_cache.append(&mut results_from_query); // merge results
            results_from_cache
        }
        e => {
            log::error!("Error while creating ping client: {:?}", e.err());
            Err( std::io::Error::from(ErrorKind::Other))?
        }
    };

    drop(permit);
    Ok(list)
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
