use serde::{Serialize, Deserialize};

use crate::contracts;
use crate::types::TxRef;
use crate::TransactionStatus;
use crate::contracts::AccountIdWrapper;

use crate::std::collections::BTreeMap;
use crate::std::string::String;

/// HelloWorld contract states.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HelloWorld {
    mnemonics: BTreeMap<AccountIdWrapper, String>,
}

/// The commands that the contract accepts from the blockchain. Also called transactions.
/// Commands are supposed to update the states of the contract.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
   SetMnemonic {
        mnemonic: String,
    },
}

/// The errors that the contract could throw for some queries
#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    NotAuthorized,
    SomeOtherError,
}

/// Query requests. The end users can only query the contract states by sending requests.
/// Queries are not supposed to write to the contract states.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    GetMnemonic,
}

/// Query responses.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    GetMnemonic {
        mnemonic: String,
    },
    /// Something wrong happened
    Error(Error)
}


impl HelloWorld {
    /// Initializes the contract
    pub fn new() -> Self {
        Default::default()
    }
}

impl contracts::Contract<Command, Request, Response> for HelloWorld {
    // Returns the contract id
    fn id(&self) -> contracts::ContractId { contracts::HELLO_WORLD }

    // Handles the commands from transactions on the blockchain. This method doesn't respond.
    fn handle_command(&mut self, _origin: &chain::AccountId, _txref: &TxRef, cmd: Command) -> TransactionStatus {
        match cmd {
            // Handle the `Increment` command with one parameter
            Command::SetMnemonic { mnemonic } => {
                
                let current_user = AccountIdWrapper(_origin.clone());
                // Insert the latest mnemonic
                self.mnemonics.insert(current_user, mnemonic);
                // Returns TransactionStatus::Ok to indicate a successful transaction
                TransactionStatus::Ok
            },
        }
    }

    // Handles a direct query and responds to the query. It shouldn't modify the contract states.
    fn handle_query(&mut self, _origin: Option<&chain::AccountId>, req: Request) -> Response {
        let inner = || -> Result<Response, Error> {
            match req {
                // Handle the `GetMnemonic` request
                Request::GetMnemonic => {
                    // Unwrap the current user account
                    if let Some(account) = _origin {
                        let current_user = AccountIdWrapper(account.clone());
                        if self.mnemonics.contains_key(&current_user) {
                            
                            let mnemonic = self.mnemonics.get(&current_user);
                            return Ok(Response::GetMnemonic { mnemonic: mnemonic.unwrap().clone() })
                        }
                    }

                    // Respond NotAuthorized when no account is specified
                    Err(Error::NotAuthorized)
                },
            }
        };
        match inner() {
            Err(error) => Response::Error(error),
            Ok(resp) => resp
        }
    }
}

