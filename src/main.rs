use block_payload::{
    PaymentInfo, StartTransportationData, 
    DeliveredTransportationData, ProductInfo, 
    MetricData
};
use chrono::Local;
use dotenv::dotenv;
use iota_sdk::{
    client::core::Client,
    types::block::{
        Block, BlockId, BlockDto, 
        payload::dto::{PayloadDto, TaggedDataPayloadDto}
    },
};
use std::{env, io, path::Path, time::{Instant, Duration}};
use rand::{rngs::ThreadRng, Rng};


mod block_payload;

mod custom_error;
use custom_error::Error;

use crate::block_payload::TaggedDataPayload;

// Try to read an environment variable. If a .env file exists, try to read from
// it first. In case the environment variable does not exist in neither the 
// .env file nor the environment, return an error.
fn read_env_var(var: String) -> Result<String, Error> {
    if Path::new(".env").exists() {
        dotenv().ok();
    }

    let value: String = match env::var(&var) {
        Ok(value) => value,
        Err(_) => return Err(Error::EnvError(env::VarError::NotPresent)),
    };

    Ok(value)
}

// Try to read the initial block id from the environment. If it does not exist,
// ask the user to input it.
fn block_id_input() -> Result<String, Error> {
    let input: String = match read_env_var("INITIAL_BLOCK_ID".to_string()) {
        Ok(value) => value,
        Err(_err) => {
            let stdin: io::Stdin = io::stdin();
            println!("Enter BlockId:");
            let mut user_input: String = String::new();
            stdin
                .read_line(&mut user_input)?;
            user_input.to_string()
        }
    };

    if !input.starts_with("0x") {
        return Err(Error::Anyhow(anyhow::Error::msg(
            "BlockId must start with 0x",
        )));
    } else if input.len() != 66 {
        return Err(Error::Anyhow(anyhow::Error::msg(
            "BlockId must be 66 characters long",
        )));
    }

    Ok(input)
}

// Create an IOTA client with the given node URL. The client will use local PoW
// with the maximum number of threads available on the machine.
async fn create_iota_client() -> Result<Client, Error> {
    let node_url: String = read_env_var("NODE_URL".to_string())?;

    let client: Client = Client::builder()
        .with_node(node_url.as_str())?
        .with_local_pow(true)
        .with_pow_worker_count(num_cpus::get())
        .finish()
        .await?;
    Ok(client)
}

async fn get_block(client: &Client, block_id: &String) -> Result<BlockDto, Error> {
    let block_id: BlockId = block_id.parse()?;

    let block: Block = client.get_block(&block_id).await?;

    let block_dto: BlockDto = BlockDto::from(&block);

    Ok(block_dto)
}

// Extract the payment info from the block payload. Only specific block types
// of our supply chain block model contain payment information.
// RawMaterialsProducerBlockData, SupplierBlockData, ManufacturerBlockData,
// DistributorBlockData, RetailerBlockData
// The other block type are not accepted as input.
fn extract_payment_info(block: BlockDto) -> Result<PaymentInfo, Error> {
    use block_payload::BlockData::*;

    let block_payload: PayloadDto = match block.payload {
        Some(payload) => payload,
        None => return Err(Error::Anyhow(anyhow::Error::msg(
            "Block has no payload"
        ))),
    };

    let tagged_data: Box<TaggedDataPayloadDto> = match block_payload {
        PayloadDto::TaggedData(data) => data,
        _ => return Err(Error::Anyhow(anyhow::Error::msg(
            "Block payload is not tagged data"
        )))
    };

    let string_data: String = String::from_utf8((*tagged_data.data).to_vec())?;

    let block_payload: TaggedDataPayload = serde_json::from_str(&string_data)?;

    let payment_info: PaymentInfo = match block_payload.data {
        RawMaterialsProducerBlockData(data) => data.payment_info,
        SupplierBlockData(data) => data.payment_info,
        ManufacturerBlockData(data) => data.payment_info,
        DistributorBlockData(data) => data.payment_info,
        RetailerBlockData(data) => data.payment_info,
        _ => return Err(Error::Anyhow(anyhow::Error::msg(
            "Block payload does not contain payment info data"
        )))
    };

    Ok(payment_info)
}

async fn post_iota_block(
    client: &Client,
    tag: Vec<u8>,
    data: Vec<u8>
) -> Result<BlockId, Error> {

    print!("--------------------------------------------------\n");
    println!("Posting block...");
    let start: Instant = Instant::now();
    
    let block: Block = client
        .build_block()
        .with_tag(tag)
        .with_data(data)
        .finish()
        .await?;
    
    let block_id: BlockId = client.post_block(&block).await?;

    println!("Block posted ---- {:?}", start.elapsed());
    print_block_on_explorer(&block_id.to_string())?;
    print!("--------------------------------------------------\n");

    Ok(block_id)
}

async fn start_transportation(
    client: &Client,
    initial_block_id: &String
) -> Result<BlockId, Error> {

    let file_cid: Option<String> = match read_env_var("START_TRANSPORTATION_CID".to_string()){
        Ok(value) => Some(value),
        Err(_err) => None
    };

    let product_info: ProductInfo = ProductInfo::new(
        String::from("Transportation Information Data"), file_cid
    );

    let start_transaction_data: StartTransportationData = 
        StartTransportationData::new(
            String::from("Transportation Company Information Data"),
            product_info,
            Local::now().to_string(),
            initial_block_id.to_owned()
        );
    
    let data: Vec<u8> = serde_json::to_string(&start_transaction_data)?
        .as_bytes()
        .to_vec();

    let tag: Vec<u8> = String::from("Start Transportation Tag").as_bytes().to_vec();

    let block_id: BlockId = post_iota_block(client, tag, data).await?;

    Ok(block_id)
}

fn print_block_on_explorer(block_id: &String) -> Result<(), Error> {
    let explorer_url: String = read_env_var("EXPLORER_URL".to_string())?;
    let block_explorer_url: String = format!("{}/block/{}", explorer_url, block_id);
    println!("Block posted on: {}", block_explorer_url);
    Ok(())
}

fn gen_random_number(min: f64, max: f64) -> Result<f64, Error>{
    let mut rng: ThreadRng = rand::thread_rng();
    let random_number: f64 = rng.gen::<f64>();

    // Specify range
    let number: f64 = min + (max - min) * random_number;

    let res: f64 = (number * 100.0).round() / 100.0;
    Ok(res)
}

async fn temperature_metric(
    client: &Client,
    previous_block_id: &String
) -> Result<BlockId, Error>{
    let metric_data: MetricData = MetricData::new(
        String::from("Temperature"),
        gen_random_number(-5.0, 30.0)?,
        String::from("Celsius"),
        Local::now().to_string(),
        previous_block_id.to_owned()
    );

    let data: Vec<u8> = serde_json::to_string(&metric_data)?
        .as_bytes()
        .to_vec();

    let tag: Vec<u8> = String::from("Temperature Metric Tag").as_bytes().to_vec();

    let block_id: BlockId = post_iota_block(client, tag, data).await?;

    Ok(block_id)
}

async fn humidity_metric(
    client: &Client,
    previous_block_id: &String
) -> Result<BlockId, Error>{
    let metric_data: MetricData = MetricData::new(
        String::from("Humidity"),
        gen_random_number(0.0, 100.0)?,
        String::from("%"),
        Local::now().to_string(),
        previous_block_id.to_owned()
    );

    let data: Vec<u8> = serde_json::to_string(&metric_data)?
        .as_bytes()
        .to_vec();

    let tag: Vec<u8> = String::from("Humidity Metric Tag").as_bytes().to_vec();

    let block_id: BlockId = post_iota_block(client, tag, data).await?;

    Ok(block_id)
}

async fn deliver_transportation(
    client: &Client,
    payment_info: PaymentInfo,
    metrics: Vec<String>
) -> Result<BlockId, Error> {
    let file_cid: Option<String> = match read_env_var("DELIVER_TRANSPORTATION_CID".to_string()){
        Ok(value) => Some(value),
        Err(_err) => None
    };

    let product_info: ProductInfo = ProductInfo::new(
        String::from("Product Delivery Information"), file_cid
    );

    let delivered_transportation_data: DeliveredTransportationData = 
        DeliveredTransportationData::new(
            product_info,
            Local::now().to_string(),
            payment_info,
            metrics
        );

    let data: Vec<u8> = serde_json::to_string(&delivered_transportation_data)?
        .as_bytes()
        .to_vec();
    
    let tag: Vec<u8> = String::from("Delivered Transportation Tag")
        .as_bytes()
        .to_vec();

    let block_id: BlockId = post_iota_block(client, tag, data).await?;

    Ok(block_id)
}

#[tokio::main]
async fn main() {
    let block_id: String = block_id_input().unwrap();

    let iota_client: Client = create_iota_client().await.unwrap();

    let initial_block: BlockDto = 
        get_block(&iota_client, &block_id)
        .await
        .unwrap();
    
    let payment_info: PaymentInfo = 
        extract_payment_info(initial_block)
        .unwrap();

    let start_transportation_block_id: BlockId =
        start_transportation(&iota_client, &block_id).await.unwrap();

    let start_time: Instant = Instant::now();
    let one_minute: Duration = Duration::from_secs(120);
    
    let mut temperature_previous_block: BlockId = start_transportation_block_id;
    let mut humidity_previous_block: BlockId = start_transportation_block_id;
    let mut metrics: Vec<String> = Vec::new();

    loop {

        match temperature_metric(&iota_client, &temperature_previous_block.to_string()).await {
            Ok(block_id) => temperature_previous_block = block_id,
            Err(err) => println!("Error: {:?}", err)
        };

        match humidity_metric(&iota_client, &humidity_previous_block.to_string()).await {
            Ok(block_id) => humidity_previous_block = block_id,
            Err(err) => println!("Error: {:?}", err)
        };

        if start_time.elapsed() >= one_minute {
            metrics.push(temperature_previous_block.to_string());
            metrics.push(humidity_previous_block.to_string());
            break;
        }
    }

    let _deliver_transportation_block_id: BlockId =
        deliver_transportation(&iota_client, payment_info, metrics)
        .await.unwrap();

}
