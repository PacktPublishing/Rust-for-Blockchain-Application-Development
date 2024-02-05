#[account] 
pub struct Message { 
    pub author: Pubkey, 
    pub timestamp: i64, 
    pub topic: String, 
    pub content: String, 
} 

// Constants defining sizes of account properties 

const DISCRIMINATOR_LENGTH: usize = 8; 
const PUBLIC_KEY_LENGTH: usize = 32; 
const TIMESTAMP_LENGTH: usize = 8; 
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string. 
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max. 
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max. 

impl Message { 
    // Constant representing the total size of the Message account 
    const LEN: usize = DISCRIMINATOR_LENGTH 
        + PUBLIC_KEY_LENGTH // Author. 
        + TIMESTAMP_LENGTH // Timestamp. 
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic. 
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content. 
}


#[derive(Accounts)] 
pub struct SendMessage<'info> { 
    #[account(init, payer = author, space = Message::LEN)] 
    pub message: Account<'info, Message>, 
    #[account(mut)] 
    pub author: Signer<'info>, 
    #[account(address = system_program::ID)] 
    pub system_program: AccountInfo<'info>, 
    pub system_program: Program<'info, System>, 
} 


#[program]
pub mod solana_custom {
    use super::*;
    pub fn send_message(ctx: Context<SendMessage>, topic: String, content: String) -> ProgramResult { 
    let message: &mut Account<Message> = &mut ctx.accounts.message; 
    let sender: &Signer = &ctx.accounts.sender; 
    let clock: Clock = Clock::get().unwrap(); 
    if topic.chars().count() > 50 { 
        return Err(ErrorCode::TopicTooLong.into()); 
    } 
    if content.chars().count() > 280 { 
        return Err(ErrorCode::ContentTooLong.into()); 
    } 
    message.sender = *sender.key; 
    message.timestamp = clock.unix_timestamp; 
    message.topic = topic; 
    message.content = content; 
    Ok(());
    }
}