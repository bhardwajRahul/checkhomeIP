use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Error};
use tokio::time::sleep;

use crate::mail;

const WAIT_TIME: u64 = 300;

pub struct CheckIP;

impl CheckIP {
    async fn check() -> Result<String, Error> {
        let apis = ["https://httpbin.org/ip", "https://api.ipify.org/?format=json", "https://api.seeip.org/jsonip"];
        tracing::debug!("Sending HTTP request...");

        for (k, v) in apis.iter().enumerate() {
            match reqwest::get(*v).await {
                Ok(resp) => {
                    match resp.json::<HashMap<String, String>>().await {
                        Ok(r) => {
                            if k == 0 {
                                if let Some(ip) = r.get("origin") {
                                    return Ok(String::from(ip));
                                } else {
                                    tracing::error!("failed to get IP from response!");
                                }
                            } else if let Some(ip) = r.get("ip") {
                                return Ok(String::from(ip));
                            } else {
                                tracing::error!("Failed to get IP from response!");
                            }
                        }
                        Err(e) =>{ tracing::error!("Failed to parse response as JSON: {}", e); }
                    }
                }
                Err(e) => tracing::error!("Failed to send request to remote server: {}", e),
            }
        }

        Err(anyhow!("Failed to complete request to all providers!"))
    }


    pub async fn init() {
        let mut old_time: u64 = 0;
        let mut current_ip: String = String::new();

        // Get current IP and store in var
        tracing::info!("Getting inital IP");
        match Self::check().await {
            Ok(new_ip) => {
                if new_ip != current_ip {
                    current_ip = new_ip.clone();
                    tracing::info!("Initial IP set: {new_ip}");
                    old_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                }
            }
            Err(e) => tracing::error!("Error: {}", e),
        }

        loop {
            if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() >= old_time + WAIT_TIME {
                match Self::check().await {
                    Ok(new_ip) => {
                        if new_ip != current_ip {
                            tracing::info!("Your home IP has changed from {current_ip} to {new_ip}");
                            mail::send_email(&current_ip, &new_ip).await;
                            current_ip = new_ip;
                        }
                        else { tracing::info!("IP hasn't changed. Ignoring") }
                    }
                    Err(e) => tracing::error!("Error: {}", e),
                }
                old_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            } else {
                tracing::debug!("Not checking IP since {WAIT_TIME} secs not passed");
                sleep(Duration::from_secs(1)).await
            }
        }
    }
}
