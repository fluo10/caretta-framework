use crate::{
    mcp::model::DeviceIdentifier,
    types::DevicePublicKey,
    util::{decode_base32, encode_base32},
};
use caretta_id::CarettaId;
use clap::Args;

#[derive(Args, Clone, Debug)]
#[group(multiple = false, required = true)]
pub struct DeviceIdentifierOptionArgs {
    // #[arg(long)]
    // id: Option<CarettaId>,
    #[arg(long)]
    public_key: Option<DevicePublicKey>,
    // #[arg(long)]
    // name: Option<String>,
}

impl From<DeviceIdentifierOptionArgs> for DeviceIdentifier {
    fn from(value: DeviceIdentifierOptionArgs) -> Self {
        match value.public_key {
            Some(x) => Self::PublicKey(x),
            _ => unreachable!("The parsed argument must be one."),
        }
    }
}
