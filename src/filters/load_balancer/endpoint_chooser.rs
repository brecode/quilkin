/*
 * Copyright 2021 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */


use std::sync::atomic::{AtomicUsize, Ordering};
use rand::{thread_rng, Rng};
use std::time::Duration;

use crate::config::UpstreamEndpoints;

crate::include_proto!("endpoint");
use self::endpoint::get_endpoint_client::GetEndpointClient;
use self::endpoint::EndpointRequest;

/// EndpointChooser chooses from a set of endpoints that a proxy is connected to.
pub trait EndpointChooser: Send + Sync {
    /// choose_endpoints asks for the next endpoint(s) to use.
    fn choose_endpoints(&self, endpoints: &mut UpstreamEndpoints);
}

/// RoundRobinEndpointChooser chooses endpoints in round-robin order.
pub struct RoundRobinEndpointChooser {
    next_endpoint: AtomicUsize,
}

impl RoundRobinEndpointChooser {
    pub fn new() -> Self {
        RoundRobinEndpointChooser {
            next_endpoint: AtomicUsize::new(0),
        }
    }
}

impl EndpointChooser for RoundRobinEndpointChooser {
    fn choose_endpoints(&self, endpoints: &mut UpstreamEndpoints) {
        println!("is this love");
        let count = self.next_endpoint.fetch_add(1, Ordering::Relaxed);
        // Note: Unwrap is safe here because the index is guaranteed to be in range.
        let num_endpoints = endpoints.size();
        endpoints.keep(count % num_endpoints)
            .expect("BUG: unwrap should have been safe because index into endpoints list should be in range");
    }
}

/// RandomEndpointChooser chooses endpoints in random order.
pub struct RandomEndpointChooser;

impl EndpointChooser for RandomEndpointChooser {
    fn choose_endpoints(&self, endpoints: &mut UpstreamEndpoints) {
        // Note: Unwrap is safe here because the index is guaranteed to be in range.
        let idx = (&mut thread_rng()).gen_range(0..endpoints.size());
        endpoints.keep(idx)
            .expect("BUG: unwrap should have been safe because index into endpoints list should be in range");
    }
}

/// ControlPlaneEndpointChooser chooses endpoints from a Control Plane API Call.
pub struct ControlPlaneEndpointChooser;

impl EndpointChooser for ControlPlaneEndpointChooser {
    fn choose_endpoints(&self, endpoints: &mut UpstreamEndpoints) {
        println!("ControlPlaneEndpointChooser");

        let result = futures::executor::block_on(send_msg());
        match result {
            Ok(_) => {
                println!("happy");
            }
            Err(e) => {
                println!("Error {:?}", e);
            }
        }
        // Note: Unwrap is safe here because the index is guaranteed to be in range.
        let idx = (&mut thread_rng()).gen_range(0..endpoints.size());
        endpoints.keep(idx)
            .expect("BUG: unwrap should have been safe because index into endpoints list should be in range");
    }
}

async fn send_msg() -> Result<(), Box<dyn std::error::Error>> {
    // creating a channel ie connection to server

    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:50051")
     .connect_timeout(Duration::from_secs(5))
     .connect()
     .await?;

    // creating gRPC client from channel
    let mut client = GetEndpointClient::new(channel);
    // creating a new Request
    let request = tonic::Request::new(
        EndpointRequest {
        req:String::from("hi")
        },
    );
    // sending request and waiting for response
    println!("sending");

    let response = client.send(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}