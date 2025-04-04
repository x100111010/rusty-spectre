use crate::imports::*;

#[derive(Default, Handler)]
#[help("Send a Spectre transaction to a public address")]
pub struct Send;

impl Send {
    async fn main(self: Arc<Self>, ctx: &Arc<dyn Context>, argv: Vec<String>, _cmd: &str) -> Result<()> {
        // address, amount, priority fee
        let ctx = ctx.clone().downcast_arc::<SpectreCli>()?;

        let account = ctx.wallet().account()?;

        if argv.len() < 2 {
            tprintln!(ctx, "Usage: send <address> <amount|--send-all> <priority fee>");
            return Ok(());
        }

        let address = Address::try_from(argv.first().unwrap().as_str())?;
        let abortable = Abortable::default();

        // get priority fee first.
        let priority_fee_sompi = try_parse_optional_spectre_as_sompi_i64(argv.get(2))?.unwrap_or(0);

        // handle --send-all
        let amount_sompi = if argv.get(1).unwrap() == "--send-all" {
            // get mature balance from account
            let balance = account.balance().ok_or_else(|| Error::Custom("Failed to retrieve account balance".into()))?;

            // estimate fee
            let fee_sompi = account
                .clone()
                .estimate(
                    PaymentDestination::PaymentOutputs(PaymentOutputs::from((address.clone(), balance.mature))),
                    Fees::ReceiverPays(0),
                    None,
                    &abortable,
                )
                .await?;

            // subtract estimated and priority fee
            balance
                .mature
                .checked_sub(fee_sompi.aggregated_fees)
                .ok_or_else(|| Error::Custom("Insufficient funds to cover the transaction fee.".into()))?
                .checked_sub(priority_fee_sompi.try_into().unwrap_or(0))
                .ok_or_else(|| Error::Custom("Insufficient funds to cover the priority fee.".into()))?
        } else {
            // parse amount if not using --send-all
            try_parse_required_nonzero_spectre_as_sompi_u64(argv.get(1))?
        };

        let outputs = PaymentOutputs::from((address.clone(), amount_sompi));
        let (wallet_secret, payment_secret) = ctx.ask_wallet_secret(Some(&account)).await?;

        // let ctx_ = ctx.clone();
        let (summary, _ids) = account
            .send(
                outputs.into(),
                priority_fee_sompi.into(),
                None,
                wallet_secret,
                payment_secret,
                &abortable,
                Some(Arc::new(move |_ptx| {
                    // tprintln!(ctx_, "Sending transaction: {}", ptx.id());
                })),
            )
            .await?;

        tprintln!(ctx, "Transaction sent - {summary}");
        tprintln!(ctx, "\nSending {} SPR to {address}, transaction IDs:", sompi_to_spectre_string(amount_sompi));
        tprintln!(ctx, "\n{}\n", _ids.into_iter().map(|a| a.to_string()).collect::<Vec<_>>().join("\n"));

        Ok(())
    }
}
