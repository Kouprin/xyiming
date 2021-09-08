use crate::*;

pub const ERR_DEPOSIT_NOT_ENOUGH: &str = "Attached deposit is not enough, expected";
pub const ERR_ACCESS_DENIED: &str = "Caller has no access, expected";
pub const ERR_STREAM_NOT_AVAILABLE: &str = "Stream not exist or terminated";
pub const ERR_PAUSE_PAUSED: &str = "Cannot pause paused stream";
pub const ERR_CANNOT_START_STREAM: &str = "Cannot start stream, invalid stream status";
pub const ERR_TEXT_FIELD_TOO_LONG: &str = "Text field is too long";
pub const ERR_CRON_CALLS_DISABLED: &str = "Cron calls disabled";
pub const ERR_NOT_NEAR_TOKEN: &str = "Only NEAR tokens allowed in this method";
pub const ERR_NOT_FT_TOKEN: &str = "Only FT tokens allowed in this method";
pub const ERR_TOKENS_MISMATCH: &str = "Tokens mismatch";
pub const ERR_INVALID_TOKEN: &str = "Invalid token name";

pub const CREATE_STREAM_DEPOSIT: Balance = 100_000_000_000_000_000_000_000; // 0.1 NEAR
pub const ONE_YOCTO: Balance = 1;
pub const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000; // 1 NEAR
pub const MAX_TEXT_FIELD: usize = 255;
pub const GAS_FOR_FT_TRANSFER: Gas = 10_000_000_000_000;

pub type StreamId = CryptoHash;
pub type TokenId = u32;

pub const NUM_TOKENS: usize = 3;
pub const NEAR_TOKEN_ID: TokenId = 0;

// TODO use DAO way to update whitelisted tokens or/and take them from ref.finance
pub const TOKENS: [&'static str; NUM_TOKENS] = ["NEAR", "DACHA", "TARAS"];

pub const TOKEN_ACCOUNTS: [&'static str; NUM_TOKENS] =
    ["", "dacha.tkn.near", "dev-1630798753809-34755859843881"];

#[derive(BorshDeserialize, BorshSerialize, PartialEq)]
pub enum StreamStatus {
    Initialized,
    Active,
    Paused,
    Interrupted,
    Finished,
}

impl StreamStatus {
    pub(crate) fn to_string(&self) -> String {
        match self {
            StreamStatus::Initialized => "INITIALIZED".to_string(),
            StreamStatus::Active => "ACTIVE".to_string(),
            StreamStatus::Paused => "PAUSED".to_string(),
            StreamStatus::Interrupted => "INTERRUPTED".to_string(),
            StreamStatus::Finished => "FINISHED".to_string(),
        }
    }

    pub(crate) fn is_terminated(&self) -> bool {
        match self {
            StreamStatus::Initialized => false,
            StreamStatus::Active => false,
            StreamStatus::Paused => false,
            StreamStatus::Interrupted => true,
            StreamStatus::Finished => true,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum ActionType {
    Init,
    Deposit(Balance),
    Withdraw(Balance),
    Refund(Balance),
    Start,
    Pause,
    Stop,
    EnableAutoDeposit,
    DisableAutoDeposit,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Action {
    pub actor: AccountId,
    pub action_type: ActionType,
    pub timestamp: Timestamp,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ActionView {
    pub actor: String,
    pub action_type: String,
    pub amount: Option<WrappedBalance>,
    pub timestamp: WrappedTimestamp,
}

impl From<&Action> for ActionView {
    fn from(a: &Action) -> Self {
        let (action_type, amount) = match a.action_type {
            ActionType::Init => ("Init".to_string(), None),
            ActionType::Deposit(amount) => ("Deposit".to_string(), Some(amount)),
            ActionType::Withdraw(amount) => ("Withdraw".to_string(), Some(amount)),
            ActionType::Refund(amount) => ("Refund".to_string(), Some(amount)),
            ActionType::Start => ("Start".to_string(), None),
            ActionType::Pause => ("Pause".to_string(), None),
            ActionType::Stop => ("Stop".to_string(), None),
            ActionType::EnableAutoDeposit => ("Auto-deposit enabled".to_string(), None),
            ActionType::DisableAutoDeposit => ("Auto-deposit disabled".to_string(), None),
        };
        Self {
            actor: a.actor.clone(),
            action_type,
            amount: amount.map(|a| a.into()),
            timestamp: a.timestamp.into(),
        }
    }
}

#[ext_contract]
pub trait ContractB {
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance;
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum CreateOrDeposit {
    Create(CreateStruct),
    Deposit(Base58CryptoHash),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateStruct {
    pub description: Option<String>,
    pub owner_id: ValidAccountId,
    pub receiver_id: ValidAccountId,
    pub token_name: String,
    pub balance: WrappedBalance,
    pub tokens_per_tick: WrappedBalance,
    pub auto_deposit_enabled: bool,
}

impl Xyiming {
    pub(crate) fn get_token_id_by_name(token_name: &String) -> Option<TokenId> {
        for x in 0..NUM_TOKENS {
            if TOKENS[x] == token_name {
                return Some(x as u32);
            }
        }
        None
    }

    pub(crate) fn get_token_name_by_id(token_id: TokenId) -> String {
        TOKENS[token_id as usize].to_string()
    }

    pub(crate) fn build_promise(
        token_id: TokenId,
        recipient: AccountId,
        amount: Balance,
    ) -> Promise {
        if token_id == NEAR_TOKEN_ID {
            Promise::new(recipient).transfer(amount)
        } else {
            contract_b::storage_deposit(
                Some(recipient.clone()),
                Some(true),
                &TOKEN_ACCOUNTS[token_id as usize],
                ONE_NEAR,
                GAS_FOR_FT_TRANSFER,
            )
            .then(ext_fungible_token::ft_transfer(
                recipient,
                U128(amount),
                None,
                &TOKEN_ACCOUNTS[token_id as usize],
                ONE_YOCTO,
                GAS_FOR_FT_TRANSFER,
            ))
        }
    }

    pub(crate) fn valid_ft_sender(sender_id: AccountId) -> bool {
        for x in 0..NUM_TOKENS {
            if TOKENS[x] == sender_id {
                // TODO check ""
                return true;
            }
        }
        if sender_id == "alice" {
            // TODO testing only
            return true;
        }
        false
    }
}
