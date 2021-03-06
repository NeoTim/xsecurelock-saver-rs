// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error;

use crate::model::{Scenario, World};

pub mod sqlite;

/// Storage for models.
// TODO(zstewart): fix sqlite storage with some thread local magic so that non-mutating methods can
// use &self instead of &mut self.
pub trait Storage {
    /// Add a new root scenario. This scenario is the new root of a family of scenarios.
    fn add_root_scenario(&mut self, world: World, score: f64) -> Result<Scenario, Box<dyn Error>>;

    /// Add a new scenario that is the child of the specified scenario
    fn add_child_scenario(
        &mut self,
        world: World,
        score: f64,
        parent: &Scenario,
    ) -> Result<Scenario, Box<dyn Error>>;

    /// Returns the number of scenarios available.
    fn num_scenarios(&mut self) -> Result<u64, Box<dyn Error>>;

    /// Gets the nth scenario, in order of score (descending, so lower indexes are higher scoring
    /// scenarios). May return None if the index is outside the number of scenarios.
    fn get_nth_scenario_by_score(&mut self, index: u64) -> Result<Option<Scenario>, Box<dyn Error>>;

    /// Removes the bottom scoring scenarios, keeping up to number_to_keep top scoring scenarios.
    /// Returns the number of scenarios pruned.
    fn keep_top_scenarios_by_score(&mut self, number_to_keep: u64) -> Result<u64, Box<dyn Error>>;
}
