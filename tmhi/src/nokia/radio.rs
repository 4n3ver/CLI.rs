use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RadioStatus {
    #[serde(rename = "cell_CA_stats_cfg")]
    pub carrier_aggregation: Vec<CarrierAggregationStatusEntries>,

    #[serde(rename = "cell_5G_stats_cfg")]
    pub nr: Vec<Status<NrStatus>>,

    #[serde(rename = "cell_LTE_stats_cfg")]
    pub lte: Vec<Status<LteStatus>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status<T> {
    #[serde(rename = "stat")]
    pub status: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LteStatus {
    #[serde(rename = "RSSICurrent")]
    pub rssi: isize,

    #[serde(rename = "SNRCurrent")]
    pub snr: isize,

    #[serde(rename = "RSRPCurrent")]
    pub rsrp: isize,

    #[serde(rename = "RSRPStrengthIndexCurrent")]
    pub rsrp_strength_index: u8,

    #[serde(rename = "PhysicalCellID")]
    pub physical_cell_id: String,

    #[serde(rename = "RSRQCurrent")]
    pub rsrq: isize,

    #[serde(rename = "DownlinkEarfcn")]
    pub downlink_earfcn: usize,

    #[serde(rename = "SignalStrengthLevel")]
    pub signal_strength_level: u8,

    #[serde(rename = "Band")]
    pub band: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NrStatus {
    #[serde(rename = "SNRCurrent")]
    pub snr: isize,

    #[serde(rename = "RSRPCurrent")]
    pub rsrp: isize,

    #[serde(rename = "RSRPStrengthIndexCurrent")]
    pub rsrp_strength_index: u8,

    #[serde(rename = "PhysicalCellID")]
    pub physsical_cell_id: String,

    #[serde(rename = "RSRQCurrent")]
    pub rsrq: isize,

    #[serde(rename = "Downlink_NR_ARFCN")]
    pub downlink_arfcn: usize,

    #[serde(rename = "SignalStrengthLevel")]
    pub signal_strength_level: u8,

    #[serde(rename = "Band")]
    pub band: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarrierAggregationStatusEntries {
    #[serde(rename = "X_ALU_COM_DLCarrierAggregationNumberOfEntries")]
    pub downlink_count: usize,

    #[serde(rename = "X_ALU_COM_ULCarrierAggregationNumberOfEntries")]
    pub uplink_count: usize,

    #[serde(rename = "ca4GDL")]
    pub downlink_4g: HashMap<usize, CarrierAggregationStatus>,

    #[serde(rename = "ca4GUL")]
    pub uplink_4g: HashMap<usize, CarrierAggregationStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CarrierAggregationStatus {
    #[serde(rename = "PhysicalCellID")]
    pub physical_cell_id: usize,

    #[serde(rename = "ScellBand")]
    pub scell_band: String,

    #[serde(rename = "ScellChannel")]
    pub scell_channel: usize,
}
