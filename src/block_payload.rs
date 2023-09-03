use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // Allows usage of camelCase in React.js and snake_case in Tauri.
pub struct BlockPayload {
    pub tag: String,
    pub data: BlockData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // Allows usage of camelCase in React.js and snake_case in Tauri.
pub struct TaggedDataPayload {
    pub block_type: String,
    pub data: BlockData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // Allows usage of camelCase in React.js and snake_case in Tauri.
#[serde(untagged)] // Disable additional object wrapping for enums
pub enum BlockData {
    BasicBlockData(String),
    RawMaterialsProducerBlockData(RawMaterialsProducerBlockData),
    SupplierBlockData(SupplierBlockData),
    ManufacturerBlockData(ManufacturerBlockData),
    DistributorBlockData(DistributorBlockData),
    RetailerBlockData(RetailerBlockData),
    ConsumerBlockData(ConsumerBlockData),
    StartTransportationData(StartTransportationData),
    DeliveredTransportationData(DeliveredTransportationData),
    MetricData(MetricData)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PaymentInfo {
    pub wallet_address: String,
    pub smr_cost: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub previous_block: String,
    pub transaction_receipt: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    pub previous_blocks: Vec<String>,
    pub transaction_receipts: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawMaterialsProducerBlockData {
    pub provider_info: String,
    pub material_info: ProductInfo,
    pub export_timestamp: String,
    pub export_location: ExportLocation,
    pub payment_info: PaymentInfo
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductInfo {
    pub info: String,
    pub file_cid: Option<String>
}

impl ProductInfo {
    pub fn new( info: String, file_cid: Option<String>) -> Self {
        Self { info, file_cid}
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExportLocation {
    pub longitude: f32,
    pub latitude: f32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplierBlockData {
    pub supplier_info: String,
    pub processed_material_info: ProductInfo,
    pub resources: Resources,
    pub payment_info: PaymentInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturerBlockData {
    pub manufacturer_info: String,
    pub product_info: ProductInfo,
    pub resources: Resources,
    pub payment_info: PaymentInfo
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DistributorBlockData {
    pub distributor_info: String,
    pub product_distribution_info: ProductInfo,
    pub resource: Resource,
    pub payment_info: PaymentInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RetailerBlockData {
    pub retailer_info: String,
    pub product_retail_info: ProductInfo,
    pub payment_info: PaymentInfo,
    pub resource: Resource,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConsumerBlockData {
    pub consumer_info: String,
    pub resource: Resource,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StartTransportationData {
    pub transportation_company_info: String,
    pub transportation_info: ProductInfo,
    pub start_timestamp: String,
    pub previous_block: String,
}

impl StartTransportationData {
    pub fn new(
        transportation_company_info: String,
        transportation_info: ProductInfo,
        start_timestamp: String,
        previous_block: String,
    ) -> Self {
        Self {
            transportation_company_info,
            transportation_info,
            start_timestamp,
            previous_block,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeliveredTransportationData {
    pub product_delivery_info: ProductInfo,
    pub delivery_timestamp: String,
    pub payment_info: PaymentInfo,
    pub metrics: Vec<String>,
}

impl DeliveredTransportationData {
    pub fn new (
        product_delivery_info: ProductInfo,
        delivery_timestamp: String,
        payment_info: PaymentInfo,
        metrics: Vec<String>,
    ) -> Self {
        Self {
            product_delivery_info,
            delivery_timestamp,
            payment_info,
            metrics,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetricData {
    pub metric_type: String,
    pub metric_value: f64,
    pub measurement_unit: String,
    pub timestamp: String,
    pub previous_block: String,
}

impl MetricData {
    pub fn new(
        metric_type: String,
        metric_value: f64,
        measurement_unit: String,
        timestamp: String,
        previous_block: String,
    ) -> Self {
        Self {
            metric_type,
            metric_value,
            measurement_unit,
            timestamp,
            previous_block,
        }
    }
}

