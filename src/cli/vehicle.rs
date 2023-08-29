use crate::cli::print_json;
use crate::vehicles::{SetChargeLimit, SetChargingAmps};
use crate::{Api, VehicleId};
use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum VehicleCommand {
    /// Get vehicle data.
    Data,

    /// Get charge state.
    ChargeState,

    /// Set charge limit.
    SetChargeLimit { percent: u8 },

    /// Set charge amps.
    SetChargingAmps { charging_amps: i64 },

    /// Start charging.
    ChargeStart,

    /// Stop charging.
    ChargeStop,
}

#[derive(Debug, Args)]
pub struct VehicleArgs {
    pub id: VehicleId,

    #[clap(subcommand)]
    pub command: VehicleCommand,
}

impl VehicleArgs {
    pub async fn run(self, api: &Api) -> miette::Result<()> {
        match self.command {
            VehicleCommand::Data => {
                print_json(api.vehicle_data(&self.id).await);
            }
            VehicleCommand::ChargeState => {
                print_json(api.charge_state(&self.id).await);
            }
            VehicleCommand::SetChargeLimit { percent } => {
                print_json(
                    api.set_charge_limit(&self.id, &SetChargeLimit { percent })
                        .await,
                );
            }
            VehicleCommand::SetChargingAmps { charging_amps } => {
                print_json(
                    api.set_charging_amps(&self.id, &SetChargingAmps { charging_amps })
                        .await,
                );
            }
            VehicleCommand::ChargeStart => {
                print_json(api.charge_start(&self.id).await);
            }
            VehicleCommand::ChargeStop => {
                print_json(api.charge_stop(&self.id).await);
            }
        }
        Ok(())
    }
}
