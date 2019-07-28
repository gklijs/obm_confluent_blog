use avro_rs::from_value;
use avro_rs::schema::Name;
use avro_rs::types::Value;
use log::warn;
use serde::{Deserialize, Serialize, Serializer};

pub trait SchemaName {
    fn get_full_schema_name(&self) -> &'static str;
    fn get_schema_name(&self) -> &'static str;
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Deserialize, Serialize)]
pub enum Atype {
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "MANUAL")]
    Manual,
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
    fn get_full_schema_name(&self) -> &'static str {
        "nl.openweb.data.ConfirmAccountCreation"
    }
    fn get_schema_name(&self) -> &'static str {
        "ConfirmAccountCreation"
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
    fn get_full_schema_name(&self) -> &'static str {
        "nl.openweb.data.AccountCreationConfirmed"
    }
    fn get_schema_name(&self) -> &'static str {
        "AccountCreationConfirmed"
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
    fn get_full_schema_name(&self) -> &'static str {
        "nl.openweb.data.AccountCreationFailed"
    }
    fn get_schema_name(&self) -> &'static str {
        "AccountCreationFailed"
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
    fn get_full_schema_name(&self) -> &'static str {
        "nl.openweb.data.ConfirmMoneyTransfer"
    }
    fn get_schema_name(&self) -> &'static str {
        "ConfirmMoneyTransfer"
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
    fn get_full_schema_name(&self) -> &'static str {
        "nl.openweb.data.MoneyTransferConfirmed"
    }
    fn get_schema_name(&self) -> &'static str {
        "MoneyTransferConfirmed"
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
    fn get_full_schema_name(&self) -> &'static str {
        "nl.openweb.data.MoneyTransferFailed"
    }
    fn get_schema_name(&self) -> &'static str {
        "MoneyTransferFailed"
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
    fn get_full_schema_name(&self) -> &'static str {
        "nl.openweb.data.BalanceChanged"
    }
    fn get_schema_name(&self) -> &'static str {
        "BalanceChanged"
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
    fn get_full_schema_name(&self) -> &'static str {
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

pub fn get_avro_data((name, value): (Name, Value)) -> Option<AvroData> {
    match name.namespace {
        Some(namespace) => match namespace.as_str() {
            "nl.openweb.data" => match name.name.as_str() {
                "ConfirmAccountCreation" => match from_value::<ConfirmAccountCreation>(&value) {
                    Ok(v) => Some(AvroData::CAC(v)),
                    Err(e) => {
                        warn! ("error getting avro data from: {:?} error: {:?}", value, e);
                        None
                    }
                },
                "AccountCreationConfirmed" => {
                    match from_value::<AccountCreationConfirmed>(&value) {
                        Ok(v) => Some(AvroData::ACC(v)),
                        Err(e) => {
                            warn! ("error getting avro data from: {:?} error: {:?}", value, e);
                            None
                        }
                    }
                }
                "AccountCreationFailed" => match from_value::<AccountCreationFailed>(&value) {
                    Ok(v) => Some(AvroData::ACF(v)),
                    Err(e) => {
                        warn! ("error getting avro data from: {:?} error: {:?}", value, e);
                        None
                    }
                },
                "ConfirmMoneyTransfer" => match from_value::<ConfirmMoneyTransfer>(&value) {
                    Ok(v) => Some(AvroData::CMT(v)),
                    Err(e) => {
                        warn! ("error getting avro data from: {:?} error: {:?}", value, e);
                        None
                    }
                },
                "MoneyTransferConfirmed" => match from_value::<MoneyTransferConfirmed>(&value) {
                    Ok(v) => Some(AvroData::MTC(v)),
                    Err(e) => {
                        warn! ("error getting avro data from: {:?} error: {:?}", value, e);
                        None
                    }
                },
                "MoneyTransferFailed" => match from_value::<MoneyTransferFailed>(&value) {
                    Ok(v) => Some(AvroData::MTF(v)),
                    Err(e) => {
                        warn! ("error getting avro data from: {:?} error: {:?}", value, e);
                        None
                    }
                },
                "BalanceChanged" => match from_value::<BalanceChanged>(&value) {
                    Ok(v) => Some(AvroData::BC(v)),
                    Err(e) => {
                        warn! ("error getting avro data from: {:?} error: {:?}", value, e);
                        None
                    }
                },
                other => {
                    warn! ("unknown data type: {}", other);
                    None
                }
            },
            _ => {
                warn! ("namespace is not 'nl.openweb.data' while that was expected");
                None
            }
        },
        None => {
            warn! ("No namespace in data while that was expected");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avro_to_from() {
        let cac = ConfirmAccountCreation{
            id: [204, 240, 237, 74, 227, 188, 75, 46, 183, 163, 122, 214, 178, 72, 118, 162],
            a_type: Atype::Auto
        };
        let serialized = avro_rs::to_value(&cac).unwrap();
        println!("serialized = {:?}", serialized);

        let deserialized = avro_rs::from_value::<ConfirmAccountCreation>(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
        panic!()
    }

    #[test]
    fn json_to_from() {
        let cac = ConfirmAccountCreation{
            id: [204, 240, 237, 74, 227, 188, 75, 46, 183, 163, 122, 214, 178, 72, 118, 162],
            a_type: Atype::Auto
        };
        let serialized = serde_json::to_string(&cac).unwrap();
        println!("serialized = {}", serialized);

        let deserialized: ConfirmAccountCreation = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
        panic!()
    }
}
