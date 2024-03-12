// Copyright 2024 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use crate::subcommands::acc_packet::AccountPacket;

use sn_client::{Client, BATCH_SIZE};

use sn_client::protocol::storage::{RegisterAddress, RetryStrategy};

use crate::subcommands::files::upload::FilesUploadOptions;
use clap::Parser;
use color_eyre::{eyre::eyre, Result};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
pub enum FoldersCmds {
    Init {
        /// The directory to initialise as a root folder, which can then be stored on the network (and kept in sync with).
        #[clap(name = "path", value_name = "PATH")]
        path: PathBuf,
        /// The hex address where the root Folder will be stored on the network (a random address will be otherwise generated by default).
        #[clap(name = "address")]
        folder_addr: Option<String>,
    },
    Upload {
        /// The location of the file(s) to upload for creating the folder on the network.
        ///
        /// Can be a file or a directory.
        #[clap(name = "path", value_name = "PATH")]
        path: PathBuf,
        /// The batch_size to split chunks into parallel handling batches
        /// during payment and upload processing.
        #[clap(long, default_value_t = BATCH_SIZE, short='b')]
        batch_size: usize,
        /// Should the file be made accessible to all. (This is irreversible)
        #[clap(long, name = "make_public", default_value = "false", short = 'p')]
        make_data_public: bool,
        /// Set the strategy to use on chunk upload failure. Does not modify the spend failure retry attempts yet.
        ///
        /// Choose a retry strategy based on effort level, from 'quick' (least effort), through 'balanced',
        /// to 'persistent' (most effort).
        #[clap(long, default_value_t = RetryStrategy::Balanced, short = 'r', help = "Sets the retry strategy on upload failure. Options: 'quick' for minimal effort, 'balanced' for moderate effort, or 'persistent' for maximum effort.")]
        retry_strategy: RetryStrategy,
    },
    Download {
        /// The hex address of a folder.
        #[clap(name = "address")]
        folder_addr: String,
        /// The name to apply to the downloaded folder.
        #[clap(name = "target folder name")]
        folder_name: OsString,
        /// The batch_size for parallel downloading
        #[clap(long, default_value_t = BATCH_SIZE , short='b')]
        batch_size: usize,
        /// Set the strategy to use on downloads failure.
        ///
        /// Choose a retry strategy based on effort level, from 'quick' (least effort), through 'balanced',
        /// to 'persistent' (most effort).
        #[clap(long, default_value_t = RetryStrategy::Quick, short = 'r', help = "Sets the retry strategy on download failure. Options: 'quick' for minimal effort, 'balanced' for moderate effort, or 'persistent' for maximum effort.")]
        retry_strategy: RetryStrategy,
    },
    /// Report any changes made to local version of files/folders (this doesn't compare it with their versions stored on the network).
    Status {
        /// Can be a file or a directory.
        #[clap(name = "path", value_name = "PATH")]
        path: PathBuf,
    },
    /// Sync up local files/folders changes with their versions stored on the network.
    Sync {
        /// Can be a file or a directory.
        #[clap(name = "path", value_name = "PATH")]
        path: PathBuf,
        /// The batch_size to split chunks into parallel handling batches
        /// during payment and upload processing.
        #[clap(long, default_value_t = BATCH_SIZE, short='b')]
        batch_size: usize,
        /// Should the file be made accessible to all. (This is irreversible)
        #[clap(long, name = "make_public", default_value = "false", short = 'p')]
        make_data_public: bool,
        /// Set the strategy to use on chunk upload failure. Does not modify the spend failure retry attempts yet.
        ///
        /// Choose a retry strategy based on effort level, from 'quick' (least effort), through 'balanced',
        /// to 'persistent' (most effort).
        #[clap(long, default_value_t = RetryStrategy::Balanced, short = 'r', help = "Sets the retry strategy on upload failure. Options: 'quick' for minimal effort, 'balanced' for moderate effort, or 'persistent' for maximum effort.")]
        retry_strategy: RetryStrategy,
    },
}

pub(crate) async fn folders_cmds(
    cmds: FoldersCmds,
    client: &Client,
    root_dir: &Path,
    verify_store: bool,
) -> Result<()> {
    match cmds {
        FoldersCmds::Init { path, folder_addr } => {
            // init path as a fresh new folder
            let root_folder_addr =
                folder_addr.and_then(|hex_str| RegisterAddress::from_hex(&hex_str).ok());
            let acc_packet =
                AccountPacket::init(client.clone(), root_dir, &path, root_folder_addr)?;
            println!("Directoy at {path:?} initialised as a root Folder, ready to track and sync changes with the network at address: {}", acc_packet.root_folder_addr().to_hex())
        }
        FoldersCmds::Upload {
            path,
            batch_size,
            make_data_public,
            retry_strategy,
        } => {
            // init path as a fresh new folder
            let mut acc_packet = AccountPacket::init(client.clone(), root_dir, &path, None)?;

            let options = FilesUploadOptions {
                make_data_public,
                verify_store,
                batch_size,
                retry_strategy,
            };
            acc_packet.sync(options).await?;

            println!(
                "\nFolder hierarchy from {path:?} uploaded successfully at {}",
                acc_packet.root_folder_addr().to_hex()
            );
        }
        FoldersCmds::Download {
            folder_addr,
            folder_name,
            batch_size,
            retry_strategy,
        } => {
            let address = RegisterAddress::from_hex(&folder_addr)
                .map_err(|err| eyre!("Failed to parse Folder address: {err}"))?;

            let download_dir = dirs_next::download_dir().unwrap_or(root_dir.to_path_buf());
            let download_folder_path = download_dir.join(folder_name.clone());
            println!(
                "Downloading onto {download_folder_path:?} from {} with batch-size {batch_size}",
                address.to_hex()
            );
            debug!(
                "Downloading onto {download_folder_path:?} from {}",
                address.to_hex()
            );

            let _acc_packet = AccountPacket::retrieve_folders(
                client,
                root_dir,
                address,
                &download_folder_path,
                batch_size,
                retry_strategy,
            )
            .await?;
        }
        FoldersCmds::Status { path } => {
            let acc_packet = AccountPacket::from_path(client.clone(), root_dir, &path)?;

            acc_packet.status()?;
        }
        FoldersCmds::Sync {
            path,
            batch_size,
            make_data_public,
            retry_strategy,
        } => {
            let mut acc_packet = AccountPacket::from_path(client.clone(), root_dir, &path)?;

            let options = FilesUploadOptions {
                make_data_public,
                verify_store,
                batch_size,
                retry_strategy,
            };
            acc_packet.sync(options).await?;
        }
    }
    Ok(())
}