use crate::imports::*;

#[derive(Default, Handler)]
#[help("Connect to a Spectre network (mainnet or testnet)")]
pub struct Connect;

impl Connect {
    async fn main(self: Arc<Self>, ctx: &Arc<dyn Context>, argv: Vec<String>, _cmd: &str) -> Result<()> {
        let ctx = ctx.clone().downcast_arc::<SpectreCli>()?;
        if let Some(wrpc_client) = ctx.wallet().try_wrpc_client().as_ref() {
            let network_id = ctx.wallet().network_id()?;

            let arg_or_server_address = argv.first().cloned().or_else(|| ctx.wallet().settings().get(WalletSettings::Server));
            let url = match arg_or_server_address.as_deref() {
                Some("public") | Some("resolver") => {
                    tprintln!(ctx, "Connecting to a public node");
                    None
                }
                None => {
                    tprintln!(ctx, "No server set, connecting to a public node");
                    None
                }
                Some(url) => {
                    Some(wrpc_client.parse_url_with_network_type(url.to_string(), network_id.into()).map_err(|e| e.to_string())?)
                }
            };

            if url.is_none() {
                tpara!(
                    ctx,
                    "Please note that public node infrastructure is community-operated and \
                    accessing it may expose your IP address to different node providers. \
                    Consider running your own node for enhanced privacy."
                );
            }

            let options = ConnectOptions { block_async_connect: true, strategy: ConnectStrategy::Fallback, url, ..Default::default() };
            wrpc_client.connect(Some(options)).await.map_err(|e| e.to_string())?;
        } else {
            terrorln!(ctx, "Unable to connect with non-wRPC client");
        }
        Ok(())
    }
}
