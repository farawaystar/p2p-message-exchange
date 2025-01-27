use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use serde_json;

#[derive(serde::Serialize, serde::Deserialize)]
struct SerializableTransaction {
    message: Vec<u8>,
    signatures: Vec<Vec<u8>>,
}

// Generate a dummy Solana transaction
pub fn create_dummy_transaction() -> String  {
    // Create dummy accounts
    let from_keypair = Keypair::new();
    let to_pubkey = Pubkey::new_unique();
    
    // Create transfer instruction
    let instruction = system_instruction::transfer(
        &from_keypair.pubkey(),
        &to_pubkey,
        1, // 1 lamport
    );

    // Build transaction
    let message = Message::new(&[instruction], Some(&from_keypair.pubkey()));
    let mut tx = Transaction::new_unsigned(message);
    
    // Sign with dummy keypair (not needed for serialization demo)
    let _ = tx.sign(&[&from_keypair], tx.message.recent_blockhash);

    // Convert to serializable format
    let serializable_tx = SerializableTransaction {
    message: bincode::serialize(&tx.message).unwrap(),
    signatures: tx.signatures.iter().map(|s| s.as_ref().to_vec()).collect(),
    };
    
    serde_json::to_string(&serializable_tx).unwrap()

}

// Helper to deserialize transaction
pub fn deserialize_transaction(data: &str) -> Result<Transaction, serde_json::Error> {
        let serializable_tx: SerializableTransaction = serde_json::from_str(data)?;
        
        Ok(Transaction {
            message: bincode::deserialize(&serializable_tx.message).unwrap(),
            signatures: serializable_tx.signatures
                .into_iter()
                .map(|sig| solana_sdk::signature::Signature::new(&sig))
                .collect(),
        })
}