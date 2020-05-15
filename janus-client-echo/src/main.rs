
struct JanusRestClient {
    client: reqwest::Client,
}

impl JanusRestClient {
    pub fn new() -> JanusRestClient {
        JanusRestClient {
            client: reqwest::Client::new(),
        }
    }

    pub async fn create_room(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let resp = self.client.post("http://10.0.0.159:8088/janus")
            .json(&serde_json::json!({
            "janus": "create",
            "transaction": "1",
        }))
            .send()
            .await?;

        let parsed_resp: serde_json::Value = resp.json().await?;
        let room_creation = parsed_resp.as_object().expect("Json was not an object");
        let room_id = room_creation.get("data")
            .expect("No data found")
            .as_object().expect("Data was not an object")
            .get("id").expect("No id found")
            .as_i64().expect("id was not a number");
        Ok(room_id)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JanusRestClient::new();
    let room_id = client.create_room().await.expect("Room creation failed");

    println!("Room id: {}", room_id);



    Ok(())
}
