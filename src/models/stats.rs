use serde::Serialize;

#[derive(Serialize)]
pub struct StatisticResponse {
    pub lot_count: i64,
    pub mycust_count: i64,
    pub mylocation_count: i64,
    pub mypic_count: i64,
    pub area_count: i64,
}