use reqwest;
use std::collections::HashMap;
use std::time::Duration;
use serde_json::Value;

const BASE_URL: &str = "https://beaconcha.in/api/v1";

struct CommId {
    epoch: String,
    slot: String,
    index: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    
    Ok(())
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
    
    let url=format!("https://beaconcha.in/api/v1/validator/stats/{}", validator_id);
    let response = reqwest::get(&url).await?;
    let text_body = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&text_body)?;
    let missed = json_data["data"]["missed_attestations"].as_u64().unwrap_or_default();
    let e=32*get_latest_epoch().await?;
    let s=get_latest_slot().await?;
    let slots=128+e-s;
    let performance = 1.0 - (missed as f64)/ (slots as f64);
    Ok(performance)
}

async fn missed_attestations(validator_id: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let url=format!("https://beaconcha.in/api/v1/validator/stats/{}", validator_id);
    let response = reqwest::get(&url).await?;
    let text_body = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&text_body)?;
    let missed = json_data["data"]["missed_attestations"].as_u64().unwrap_or_default();
    Ok(missed)
}

async fn validator_committee_pr(cid: CommId) -> Result<f64, Box<dyn std::error::Error>>
{
    let url = format!("https://docs-demo.quiknode.pro/eth/v1/beacon/states/head/committees?epoch={}&index={}&slot={}",cid.epoch,cid.index,cid.slot);
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
    let mut missed=0;
    for validator in &validators
    {
        let mut m=missed_attestations(validator).await?;
        missed=missed+m;
    }
    let e=32*get_latest_epoch().await?;
    let s=get_latest_slot().await?;
    let vs=validators.len();
    let slots=(128+e-s)*(vs as u64);
    let performance = 1.0 - (missed as f64)/ (slots as f64);
    Ok(performance)
}