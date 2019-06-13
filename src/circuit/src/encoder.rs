//use super::{DepositBlock, TransferBlock, ExitBlock};
use crate::{CircuitDepositRequest, CircuitExitRequest, CircuitTransferTx};
use models::plasma::block::{Block, BlockData};
use models::plasma::circuit::utils::be_bit_vector_into_bytes;

fn convert_transfer(
    transactions: &Vec<models::plasma::tx::TransferTx>,
) -> Result<Vec<Vec<bool>>, String> {
    transactions
        .iter()
        .map(|tx| CircuitTransferTx::try_from(tx).map(|tx| tx.public_data_into_bits()))
        .collect()
}

fn convert_deposit(
    transactions: &Vec<models::plasma::tx::DepositTx>,
) -> Result<Vec<Vec<bool>>, String> {
    transactions
        .iter()
        .map(|tx| CircuitDepositRequest::try_from(tx).map(|tx| tx.public_data_into_bits()))
        .collect()
}

fn convert_exit(transactions: &Vec<models::plasma::tx::ExitTx>) -> Result<Vec<Vec<bool>>, String> {
    transactions
        .iter()
        .map(|tx| CircuitExitRequest::try_from(tx).map(|tx| tx.public_data_into_bits()))
        .collect()
}

pub fn encode_transactions(block: &Block) -> Result<Vec<u8>, String> {
    let mut encoding: Vec<u8> = vec![];

    let transactions_bits: Vec<Vec<bool>> = match &block.block_data {
        BlockData::Transfer {
            transactions,
            total_fees: _,
        } => convert_transfer(transactions)?,
        BlockData::Deposit {
            transactions,
            batch_number: _,
        } => convert_deposit(transactions)?,
        BlockData::Exit {
            transactions,
            batch_number: _,
        } => convert_exit(transactions)?,
    };

    for tx_bits in transactions_bits {
        let tx_encoding = be_bit_vector_into_bytes(&tx_bits);
        encoding.extend(tx_encoding.into_iter());
    }

    Ok(encoding)
}
