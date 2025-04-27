#[derive(Debug)]
pub struct CreateMerchantInput {
    pub name: String,
    pub biz_number: String,
    pub address: Option<String>,
    pub phone: Option<String>,
}
