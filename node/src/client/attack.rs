// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use indexmap::IndexMap;
use snarkos_node_sync::locators::BlockLocators;
use snarkvm::ledger::store::ConsensusStorage;
use snarkvm::prelude::*;
use rand::thread_rng;

use crate::Client;


impl<N: Network, C: ConsensusStorage<N>> Client<N, C> {
    pub fn get_patched_block_locators(&self) -> Result<BlockLocators<N>> {
        info!("@@@@[get_patched_block_locators] start");
        // Patch the block locators
        const NUM_RECENT_BLOCKS: usize = 100;
        const CHECKPOINT_INTERVAL: u32 = 10_000;
        let latest_height = self.ledger.latest_height();

        // Mock height at latest_height + 100
        let mock_latest_height = latest_height + 1000000;

        // Initialize the recents map.
        let mut recents = IndexMap::with_capacity(NUM_RECENT_BLOCKS);
        // Retrieve the recent block hashes.
        let mut rng = thread_rng();
        for height in mock_latest_height.saturating_sub((NUM_RECENT_BLOCKS - 1) as u32)..=mock_latest_height - 1 {
            recents.insert(height, <N::BlockHash>::rand(&mut rng));
        }
        recents.insert(mock_latest_height, <N::BlockHash>::rand(&mut rng));

        let mut checkpoints = IndexMap::with_capacity((mock_latest_height - 1 / CHECKPOINT_INTERVAL + 1).try_into().unwrap());
        for height in (0..=mock_latest_height - 1).step_by(CHECKPOINT_INTERVAL as usize) {
            checkpoints.insert(height, <N::BlockHash>::rand(&mut rng));
        }
        let block_locators = BlockLocators::new(recents, checkpoints).unwrap();
        *self.block_locators.lock() = Some(block_locators.clone());
        // Constru()ct the block locators.
        Ok(block_locators)
    }
}