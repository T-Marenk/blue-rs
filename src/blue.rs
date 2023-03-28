use bluer::Adapter;

/// Holds the data used to interact with bluetooth devices
pub struct Blue {
    adapter: Adapter,
    pub status: bool,
}

impl Blue {
    /// Creates new instance of Blue
    /// creates new session and adapter in order to communicate with devices
    /// as well as checks the status of the adapter
    pub async fn new() -> bluer::Result<Blue> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        let status = adapter.is_powered().await?;

        Ok(Blue { adapter, status })
    }

    /// Toggle bluetooth adapter on and off
    pub async fn toggle(&mut self) -> bluer::Result<()> {
        self.adapter.set_powered(!self.status).await?;
        self.status = !self.status;
        Ok(())
    }
}
