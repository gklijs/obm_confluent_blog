use avro_rs::schema::Name;
use avro_rs::types::Value;
use log::warn;
use serde::{Deserialize, Serialize, Serializer};

const OPEN_WEB_DATA_NAMESPACE: &str = "nl.openweb.data";
const CAC_NAME: &str = "ConfirmAccountCreation";
const ACC_NAME: &str = "AccountCreationConfirmed";
const ACF_NAME: &str = "AccountCreationFailed";
const CMT_NAME: &str = "ConfirmMoneyTransfer";
const MTC_NAME: &str = "MoneyTransferConfirmed";
const MTF_NAME: &str = "MoneyTransferFailed";
const BC_NAME: &str = "BalanceChanged";

pub trait SchemaName {
    fn get_full_schema_name(&self) -> String;
    fn get_schema_name(&self) -> &'static str;
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Deserialize, Serialize)]
pub enum Atype {
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "MANUAL")]
    Manual,
}

impl Default for Atype {
    fn default() -> Self {
        Atype::Auto
    }
}

pub type Uuid = [u8; 16];

#[serde(default)]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct ConfirmAccountCreation {
    pub id: Uuid,
    pub a_type: Atype,
}

impl Default for ConfirmAccountCreation {
    fn default() -> ConfirmAccountCreation {
        ConfirmAccountCreation {
            id: Uuid::default(),
            a_type: Atype::Auto,
        }
    }
}

impl SchemaName for ConfirmAccountCreation {
    fn get_full_schema_name(&self) -> String {
        format!("{}.{}", OPEN_WEB_DATA_NAMESPACE, CAC_NAME)
    }
    fn get_schema_name(&self) -> &'static str {
        CAC_NAME
    }
}

#[serde(default)]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AccountCreationConfirmed {
    pub id: Uuid,
    pub iban: String,
    pub token: String,
    pub a_type: Atype,
}

impl Default for AccountCreationConfirmed {
    fn default() -> AccountCreationConfirmed {
        AccountCreationConfirmed {
            id: Uuid::default(),
            iban: String::default(),
            token: String::default(),
            a_type: Atype::Auto,
        }
    }
}

impl SchemaName for AccountCreationConfirmed {
    fn get_full_schema_name(&self) -> String {
        format!("{}.{}", OPEN_WEB_DATA_NAMESPACE, ACC_NAME)
    }
    fn get_schema_name(&self) -> &'static str {
        ACC_NAME
    }
}

#[serde(default)]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct AccountCreationFailed {
    pub id: Uuid,
    pub reason: String,
}

impl Default for AccountCreationFailed {
    fn default() -> AccountCreationFailed {
        AccountCreationFailed {
            id: Uuid::default(),
            reason: String::default(),
        }
    }
}

impl SchemaName for AccountCreationFailed {
    fn get_full_schema_name(&self) -> String {
        format!("{}.{}", OPEN_WEB_DATA_NAMESPACE, ACF_NAME)
    }
    fn get_schema_name(&self) -> &'static str {
        ACF_NAME
    }
}

#[serde(default)]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct ConfirmMoneyTransfer {
    pub id: Uuid,
    pub token: String,
    pub amount: i64,
    pub from: String,
    pub to: String,
    pub description: String,
}

impl Default for ConfirmMoneyTransfer {
    fn default() -> ConfirmMoneyTransfer {
        ConfirmMoneyTransfer {
            id: Uuid::default(),
            token: String::default(),
            amount: 0,
            from: String::default(),
            to: String::default(),
            description: String::default(),
        }
    }
}

impl SchemaName for ConfirmMoneyTransfer {
    fn get_full_schema_name(&self) -> String {
        format!("{}.{}", OPEN_WEB_DATA_NAMESPACE, CMT_NAME)
    }
    fn get_schema_name(&self) -> &'static str {
        CMT_NAME
    }
}

#[serde(default)]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct MoneyTransferConfirmed {
    pub id: Uuid,
}

impl Default for MoneyTransferConfirmed {
    fn default() -> MoneyTransferConfirmed {
        MoneyTransferConfirmed {
            id: Uuid::default(),
        }
    }
}

impl SchemaName for MoneyTransferConfirmed {
    fn get_full_schema_name(&self) -> String {
        format!("{}.{}", OPEN_WEB_DATA_NAMESPACE, MTC_NAME)
    }
    fn get_schema_name(&self) -> &'static str {
        MTC_NAME
    }
}

#[serde(default)]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct MoneyTransferFailed {
    pub id: Uuid,
    pub reason: String,
}

impl Default for MoneyTransferFailed {
    fn default() -> MoneyTransferFailed {
        MoneyTransferFailed {
            id: Uuid::default(),
            reason: String::default(),
        }
    }
}

impl SchemaName for MoneyTransferFailed {
    fn get_full_schema_name(&self) -> String {
        format!("{}.{}", OPEN_WEB_DATA_NAMESPACE, MTF_NAME)
    }
    fn get_schema_name(&self) -> &'static str {
        MTF_NAME
    }
}

#[serde(default)]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct BalanceChanged {
    pub iban: String,
    pub new_balance: i64,
    pub changed_by: i64,
    pub from_to: String,
    pub description: String,
}

impl Default for BalanceChanged {
    fn default() -> BalanceChanged {
        BalanceChanged {
            iban: String::default(),
            new_balance: 0,
            changed_by: 0,
            from_to: String::default(),
            description: String::default(),
        }
    }
}

impl SchemaName for BalanceChanged {
    fn get_full_schema_name(&self) -> String {
        format!("{}.{}", OPEN_WEB_DATA_NAMESPACE, BC_NAME)
    }
    fn get_schema_name(&self) -> &'static str {
        BC_NAME
    }
}

#[derive(Debug)]
pub enum AvroData {
    CAC(ConfirmAccountCreation),
    ACC(AccountCreationConfirmed),
    ACF(AccountCreationFailed),
    CMT(ConfirmMoneyTransfer),
    MTC(MoneyTransferConfirmed),
    MTF(MoneyTransferFailed),
    BC(BalanceChanged),
}

impl Serialize for AvroData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            AvroData::CAC(v) => v.serialize(serializer),
            AvroData::ACC(v) => v.serialize(serializer),
            AvroData::ACF(v) => v.serialize(serializer),
            AvroData::CMT(v) => v.serialize(serializer),
            AvroData::MTC(v) => v.serialize(serializer),
            AvroData::MTF(v) => v.serialize(serializer),
            AvroData::BC(v) => v.serialize(serializer),
        }
    }
}

impl SchemaName for AvroData {
    fn get_full_schema_name(&self) -> String {
        match self {
            AvroData::CAC(v) => v.get_full_schema_name(),
            AvroData::ACC(v) => v.get_full_schema_name(),
            AvroData::ACF(v) => v.get_full_schema_name(),
            AvroData::CMT(v) => v.get_full_schema_name(),
            AvroData::MTC(v) => v.get_full_schema_name(),
            AvroData::MTF(v) => v.get_full_schema_name(),
            AvroData::BC(v) => v.get_full_schema_name(),
        }
    }

    fn get_schema_name(&self) -> &'static str {
        match self {
            AvroData::CAC(v) => v.get_schema_name(),
            AvroData::ACC(v) => v.get_schema_name(),
            AvroData::ACF(v) => v.get_schema_name(),
            AvroData::CMT(v) => v.get_schema_name(),
            AvroData::MTC(v) => v.get_schema_name(),
            AvroData::MTF(v) => v.get_schema_name(),
            AvroData::BC(v) => v.get_schema_name(),
        }
    }
}

fn to_uuid(bytes: &[u8]) -> Uuid {
    let mut array = [0; 16];
    let bytes = &bytes[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}

fn to_cac(value: &Value) -> Option<AvroData> {
    match value {
        Value::Record(values) => {
            let id = match &values[0] {
                (_, Value::Fixed(16, id_value)) => to_uuid(&id_value),
                _ => Uuid::default(),
            };
            let a_type = match &values[1] {
                (_, Value::Enum(1, _)) => Atype::Manual,
                _ => Atype::Auto,
            };
            Some(AvroData::CAC(ConfirmAccountCreation { id, a_type }))
        }
        _ => None,
    }
}

fn to_acc(value: &Value) -> Option<AvroData> {
    match value {
        Value::Record(values) => {
            let id = match &values[0] {
                (_, Value::Fixed(16, id_value)) => to_uuid(&id_value),
                _ => Uuid::default(),
            };
            let iban = match &values[1] {
                (_, Value::String(v)) => v.clone(),
                _ => String::default(),
            };
            let token = match &values[2] {
                (_, Value::String(v)) => v.clone(),
                _ => String::default(),
            };
            let a_type = match &values[3] {
                (_, Value::Enum(1, _)) => Atype::Manual,
                _ => Atype::Auto,
            };
            Some(AvroData::ACC(AccountCreationConfirmed {
                id,
                iban,
                token,
                a_type,
            }))
        }
        _ => None,
    }
}

fn to_acf(value: &Value) -> Option<AvroData> {
    match value {
        Value::Record(values) => {
            let id = match &values[0] {
                (_, Value::Fixed(16, id_value)) => to_uuid(&id_value),
                _ => Uuid::default(),
            };
            let reason = match &values[1] {
                (_, Value::String(v)) => v.clone(),
                _ => String::default(),
            };
            Some(AvroData::ACF(AccountCreationFailed { id, reason }))
        }
        _ => None,
    }
}

fn to_cmt(value: &Value) -> Option<AvroData> {
    match value {
        Value::Record(values) => {
            let id = match &values[0] {
                (_, Value::Fixed(16, id_value)) => to_uuid(&id_value),
                _ => Uuid::default(),
            };
            let token = match &values[1] {
                (_, Value::String(v)) => v.clone(),
                _ => String::default(),
            };
            let amount = match &values[2] {
                (_iban, Value::Long(v)) => *v,
                _ => 0,
            };
            let from = match &values[3] {
                (_, Value::String(v)) => v.clone(),
                _ => String::default(),
            };
            let to = match &values[4] {
                (_, Value::String(v)) => v.clone(),
                _ => String::default(),
            };
            let description = match &values[5] {
                (_, Value::String(v)) => v.clone(),
                _ => String::default(),
            };
            Some(AvroData::CMT(ConfirmMoneyTransfer {
                id,
                token,
                amount,
                from,
                to,
                description,
            }))
        }
        _ => None,
    }
}

fn to_mtc(value: &Value) -> Option<AvroData> {
    match value {
        Value::Record(values) => {
            let id = match &values[0] {
                (_, Value::Fixed(16, id_value)) => to_uuid(&id_value),
                _ => Uuid::default(),
            };
            Some(AvroData::MTC(MoneyTransferConfirmed { id }))
        }
        _ => None,
    }
}

fn to_mtf(value: &Value) -> Option<AvroData> {
    match value {
        Value::Record(values) => {
            let id = match &values[0] {
                (_, Value::Fixed(16, id_value)) => to_uuid(&id_value),
                _ => Uuid::default(),
            };
            let reason = match &values[1] {
                (_, Value::String(v)) => v.clone(),
                _ => String::default(),
            };
            Some(AvroData::MTF(MoneyTransferFailed { id, reason }))
        }
        _ => None,
    }
}

fn to_bc(value: &Value) -> Option<AvroData> {
    match value {
        Value::Record(values) => {
            let iban = match &values[0] {
                (_, Value::String(v)) => v.clone(),
                _ => String::default(),
            };
            let new_balance = match &values[1] {
                (_iban, Value::Long(v)) => *v,
                _ => 0,
            };
            let changed_by = match &values[2] {
                (_iban, Value::Long(v)) => *v,
                _ => 0,
            };
            let from_to = match &values[3] {
                (_, Value::String(v)) => v.clone(),
                _ => String::default(),
            };
            let description = match &values[4] {
                (_, Value::String(v)) => v.clone(),
                _ => String::default(),
            };
            Some(AvroData::BC(BalanceChanged {
                iban,
                new_balance,
                changed_by,
                from_to,
                description,
            }))
        }
        _ => None,
    }
}

pub fn get_avro_data((name, value): (Name, Value)) -> Option<AvroData> {
    match name.namespace {
        Some(namespace) => match namespace.as_str() {
            OPEN_WEB_DATA_NAMESPACE => match name.name.as_str() {
                CAC_NAME => to_cac(&value),
                ACC_NAME => to_acc(&value),
                ACF_NAME => to_acf(&value),
                CMT_NAME => to_cmt(&value),
                MTC_NAME => to_mtc(&value),
                MTF_NAME => to_mtf(&value),
                BC_NAME => to_bc(&value),
                other => {
                    warn!("unknown data type: {}", other);
                    None
                }
            },
            _ => {
                warn!("namespace is not 'nl.openweb.data' while that was expected");
                None
            }
        },
        None => {
            warn!("No namespace in data while that was expected");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avro_to_from() {
        let cac = ConfirmAccountCreation {
            id: [
                204, 240, 237, 74, 227, 188, 75, 46, 183, 163, 122, 214, 178, 72, 118, 162,
            ],
            a_type: Atype::Auto,
        };
        let serialized = avro_rs::to_value(&cac).unwrap();
        println!("serialized = {:?}", serialized);

        let deserialized = avro_rs::from_value::<ConfirmAccountCreation>(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
        panic!()
    }

    #[test]
    fn json_to_from() {
        let cac = ConfirmAccountCreation {
            id: [
                204, 240, 237, 74, 227, 188, 75, 46, 183, 163, 122, 214, 178, 72, 118, 162,
            ],
            a_type: Atype::Auto,
        };
        let serialized = serde_json::to_string(&cac).unwrap();
        println!("serialized = {}", serialized);

        let deserialized: ConfirmAccountCreation = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
        panic!()
    }
}
