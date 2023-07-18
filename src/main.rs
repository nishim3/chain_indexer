use warp::{self, Filter};
use reqwest;
use std::collections::HashMap;
use std::time::Duration;
use serde_json::Value;
use tokio;

const BASE_URL: &str = "https://beaconcha.in/api/v1";

#[derive(Debug, Clone)]
struct CommId {
    epoch: String,
    slot: String,
    index: String,
}

#[tokio::main]
async fn main() {
    let chain_index_task = tokio::spawn(async move {
        chain_index().await.expect("Error running chain_index");
    });

    let validator_pr_route = warp::path!("validator" / String)
        .and_then(|validator_id: String| async move {
            let performance = validator_pr(&validator_id).await.unwrap();
            Ok::<_, warp::Rejection>(format!("Performance:{} %\n",(100.0*(performance as f32)).to_string()))
        });

    let validator_committee_pr_route = warp::path!("validator_committee" / String)
        .and_then(|comm_id: String| async move {
            let parts: Vec<&str> = comm_id.split('_').collect();
            if parts.len() != 3 {
                return Err(warp::reject::not_found());
            }
            let epoch = parts[0].to_string();
            let slot = parts[1].to_string();
            let index = parts[2].to_string();
            let performance = validator_committee_pr(CommId { epoch, slot, index })
                .await
                .unwrap();
            Ok::<_, warp::Rejection>(format!("Performance:{}\n",(100.0*(performance as f32)).to_string()))
        });

    let routes = validator_pr_route.or(validator_committee_pr_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    chain_index_task.await.unwrap();
}

async fn chain_index() -> Result<(), Box<dyn std::error::Error>> {
    let mut indexed_slots: HashMap<u64, u64> = HashMap::new();
    let mut latest_epoch = get_latest_epoch().await?;

    loop {
        let current_epoch = get_latest_epoch().await?;

        if current_epoch > latest_epoch {
            let old_epochs: Vec<u64> = indexed_slots
                .keys()
                .cloned()
                .filter(|&epoch| epoch < current_epoch - 4)
                .collect();
            for epoch in old_epochs {
                indexed_slots.remove(&epoch);
            }

            latest_epoch = current_epoch;
        }

        let slot = get_latest_slot().await?;
        indexed_slots.insert(latest_epoch, slot);

        println!("Indexed Slots: {:?}", indexed_slots);

        tokio::time::sleep(Duration::from_secs(12)).await;
    }
}

async fn get_latest_epoch() -> Result<u64, Box<dyn std::error::Error>> {
    let url = format!("{}/epoch/latest", BASE_URL);

    let response = reqwest::get(&url).await?;
    let text_body = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&text_body)?;
    let epoch = json_data["data"]["epoch"].as_u64().unwrap_or_default();

    Ok(epoch)
}

async fn get_latest_slot() -> Result<u64, Box<dyn std::error::Error>> {
    let url = format!("{}/slot/latest", BASE_URL);

    let response = reqwest::get(&url).await?;
    let text_body = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&text_body)?;
    let slot = json_data["data"]["slot"].as_u64().unwrap_or_default();

    Ok(slot)
}

async fn validator_pr(validator_id: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("{}/validator/stats/{}", BASE_URL, validator_id);

    let response = reqwest::get(&url).await?;
    let text_body = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&text_body)?;
    let missed = json_data["data"]["missed_attestations"].as_u64().unwrap_or_default();
    let e = 32 * get_latest_epoch().await?;
    let s = get_latest_slot().await?;
    let slots = 128 + e - s;
    let performance = 1.0 - (missed as f64) / (slots as f64);
    Ok(performance)
}

async fn missed_attestations(validator_id: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let url = format!("{}/validator/stats/{}", BASE_URL, validator_id);

    let response = reqwest::get(&url).await?;
    let text_body = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&text_body)?;
    let missed = json_data["data"]["missed_attestations"].as_u64().unwrap_or_default();
    Ok(missed)
}

async fn validator_committee_pr(cid: CommId) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!(
        "https://docs-demo.quiknode.pro/eth/v1/beacon/states/head/committees?epoch={}&index={}&slot={}",
        cid.epoch, cid.index, cid.slot
    );

    let response = reqwest::get(&url).await?;
    let text_body = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&text_body)?;
    let validators = match &json_data["data"][0]["validators"] {
        Value::Array(arr) => arr
            .iter()
            .filter_map(|val| val.as_str())
            .collect::<Vec<&str>>(),
        _ => Vec::new(),
    };
    let mut missed = 0;
    for validator in &validators {
        let m = missed_attestations(validator).await?;
        missed += m;
    }
    let e = 32 * get_latest_epoch().await?;
    let s = get_latest_slot().await?;
    let vs = validators.len();
    let slots = (128 + e - s) * (vs as u64);
    let performance = 1.0 - (missed as f64) / (slots as f64);
    Ok(performance)
}