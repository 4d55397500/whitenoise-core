use std::collections::HashMap;
use crate::utilities::constraint::{Constraint, NodeConstraints};

use crate::base;
use crate::proto;
use crate::hashmap;
use crate::components::Component;
use crate::utilities::constraint;

impl Component for proto::RowMin {
    // modify min, max, n, categories, is_public, non-null, etc. based on the arguments and component
    fn propagate_constraint(
        &self,
        constraints: &NodeConstraints,
    ) -> Result<Constraint, String> {
        Ok(Constraint {
            nullity: false,
            releasable: false,
            nature: None,
            num_records: None
        })
    }

    fn is_valid(
        &self,
        constraints: &NodeConstraints,
    ) -> bool {
        false
    }

    fn expand_graph(
        &self,
        privacy_definition: &proto::PrivacyDefinition,
        component: &proto::Component,
        maximum_id: u32,
        component_id: u32,
        constraints: &NodeConstraints,
    ) -> Result<(u32, HashMap<u32, proto::Component>), String> {
        Ok((maximum_id, hashmap![component_id => component.to_owned()]))
    }

    fn compute_sensitivity(
        &self,
        privacy_definition: &proto::PrivacyDefinition,
        constraints: &NodeConstraints,
    ) -> Option<Vec<f64>> {
        None
    }

    fn accuracy_to_privacy_usage(
        &self,
        privacy_definition: &proto::PrivacyDefinition,
        constraints: &NodeConstraints,
        accuracy: &proto::Accuracy,
    ) -> Option<proto::PrivacyUsage> {
        None
    }

    fn privacy_usage_to_accuracy(
        &self,
        privacy_definition: &proto::PrivacyDefinition,
        constraint: &NodeConstraints,
    ) -> Option<f64> {
        None
    }

    fn summarize(
        &self,
        constraints: &NodeConstraints,
    ) -> String {
        "".to_string()
    }
}