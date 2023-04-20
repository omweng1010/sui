// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use std::sync::Arc;
use sui_core::authority::test_authority_builder::TestAuthorityBuilder;
use sui_core::authority::AuthorityState;
use sui_core::test_utils::init_state;
use sui_move_build::BuildConfig;

struct SurferState {
    state: Arc<AuthorityState>,
}

impl SurferState {
    pub fn new() -> Self {
        Self {
            state: init_state(),
        }
    }

    pub fn load_package_from_path(path: PathBuf) {}
}
