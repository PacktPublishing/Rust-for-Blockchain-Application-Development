// --------------------------------------------------------------------------------------------------
// Structs
// --------------------------------------------------------------------------------------------------

pub struct Block {
    timestamp: i64,                
    pre_block_hash: String,         
    hash: String,                   
    transactions: Vec<Transaction>, 
    nonce: i64,                     
    height: usize,                  
}

pub struct Transaction {
    id: Vec<u8>,         
    vin: Vec<TXInput>,   
    vout: Vec<TXOutput>, 
}

pub struct TXInput {
    txid: Vec<u8>,      
    vout: usize,       
    signature: Vec<u8>,
    pub_key: Vec<u8>,  
}

pub struct TXOutput {
    value: i32,            
    pub_key_hash: Vec<u8>,
}

pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>, // hash of last block
    db: Db,
}

pub struct Wallet {
    pkcs8: Vec<u8>,
    public_key: Vec<u8>,
}

pub struct Node {
    addr: String,
}
pub struct Nodes {
    inner: RwLock<Vec<Node>>,
}

pub struct Server {
    blockchain: Blockchain,
}

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}
