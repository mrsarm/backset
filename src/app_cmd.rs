use crate::app_state::AppState;
use crate::conf::Config;
use crate::core::Result;
use crate::page::QuerySearch;
use crate::tenants::model::Tenant;
use log::info;

pub struct AppCmd {
    pub state: AppState,
}

impl AppCmd {
    pub async fn build(config: Config) -> Self {
        let state = AppState::new(config).await;
        AppCmd { state }
    }

    pub async fn list_tenants(&self, q: QuerySearch) -> Result<()> {
        let mut tx = self.state.get_tx().await?;
        let tenants = Tenant::find(&mut tx, &q).await?;
        for tenant in tenants.iter() {
            info!("{}: {}", tenant.id, tenant.name);
        }
        Ok(())
    }
}