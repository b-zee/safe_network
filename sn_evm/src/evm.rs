// Copyright 2024 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

pub use evmlib::utils::{DATA_PAYMENTS_ADDRESS, PAYMENT_TOKEN_ADDRESS, RPC_URL};

/// Load the evm network from env
pub use evmlib::utils::network_from_env;

/// Load the evm network from local CSV
pub use evmlib::utils::local_evm_network_from_csv;