// Copyright 2024 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use ant_evm::AttoTokens;
use ant_protocol::{NetworkAddress, PrettyPrintRecordKey};
use thiserror::Error;

pub(super) type Result<T, E = Error> = std::result::Result<T, E>;

/// Internal error.
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Network error {0}")]
    Network(#[from] ant_networking::NetworkError),

    #[error("Protocol error {0}")]
    Protocol(#[from] ant_protocol::Error),

    #[error("Register error {0}")]
    Register(#[from] ant_registers::Error),

    #[error("Transfers Error {0}")]
    Transfers(#[from] ant_evm::EvmError),

    #[error("Failed to parse NodeEvent")]
    NodeEventParsingFailed,

    #[error("Failed to send a cmd to the node: {0}")]
    NodeCmdFailed(String),

    #[error("Overflow occurred while adding values")]
    NumericOverflow,

    // ---------- Record Errors
    #[error("Record was not stored as no payment supplied: {0:?}")]
    InvalidPutWithoutPayment(PrettyPrintRecordKey<'static>),
    /// At this point in replication flows, payment is unimportant and should not be supplied
    #[error("Record should not be a `WithPayment` type: {0:?}")]
    UnexpectedRecordWithPayment(PrettyPrintRecordKey<'static>),
    // The Record::key must match with the one that is derived from the Record::value
    #[error("The Record::key does not match with the key derived from Record::value")]
    RecordKeyMismatch,

    // Scratchpad is old version
    #[error("A newer version of this Scratchpad already exists")]
    IgnoringOutdatedScratchpadPut,
    // Scratchpad is invalid
    #[error("Scratchpad signature is invalid over the counter + content hash")]
    InvalidScratchpadSignature,

    // ---------- Payment Errors
    #[error("The content of the payment quote is invalid")]
    InvalidQuoteContent,
    #[error("The payment quote's signature is invalid")]
    InvalidQuoteSignature,
    #[error("The payment quote expired for {0:?}")]
    QuoteExpired(NetworkAddress),
    /// Payment proof received has no inputs
    #[error(
        "Payment proof received with record:{0:?}. No payment for our node in its transaction"
    )]
    NoPaymentToOurNode(PrettyPrintRecordKey<'static>),
    /// Missing network royalties payment
    #[error("Missing network royalties payment in proof received with record: {0:?}.")]
    NoNetworkRoyaltiesPayment(PrettyPrintRecordKey<'static>),
    /// The amount paid by payment proof is not the required for the received content
    #[error("The amount paid by payment proof is not the required for the received content, paid {paid}, expected {expected}")]
    PaymentProofInsufficientAmount {
        paid: AttoTokens,
        expected: AttoTokens,
    },
    #[error("A payment we received contains cash notes already confirmed to be spent")]
    ReusedPayment,

    // ---------- Initialize Errors
    #[error("Failed to generate a reward key")]
    FailedToGenerateRewardKey,

    // ---------- Miscellaneous Errors
    #[error("Failed to obtain node's current port")]
    FailedToGetNodePort,
    /// The request is invalid or the arguments of the function are invalid
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    /// Error occurred in an async thread
    #[error("Error occured in async thread: {0}")]
    JoinErrorInAsyncThread(String),

    #[error("EVM Network error: {0}")]
    EvmNetwork(String),
    #[error("Invalid quote timestamp: {0}")]
    InvalidQuoteTimestamp(String),
}