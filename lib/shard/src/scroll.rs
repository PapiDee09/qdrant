use schemars::JsonSchema;
use segment::data_types::load_profile::LoadProfile;
use segment::data_types::order_by::OrderByInterface;
use segment::types::{Filter, PointIdType, WithPayloadInterface, WithVector};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Scroll request - paginate over all points which matches given condition
#[derive(Clone, Debug, PartialEq, Hash, Deserialize, Serialize, JsonSchema, Validate)]
#[serde(rename_all = "snake_case")]
pub struct ScrollRequestInternal {
    /// Start ID to read points from.
    pub offset: Option<PointIdType>,

    /// Page size. Default: 10
    #[validate(range(min = 1))]
    pub limit: Option<usize>,

    /// Look only for points which satisfies this conditions. If not provided - all points.
    #[validate(nested)]
    pub filter: Option<Filter>,

    /// Select which payload to return with the response. Default is true.
    pub with_payload: Option<WithPayloadInterface>,

    /// Options for specifying which vectors to include into response. Default is false.
    #[serde(default, alias = "with_vectors")]
    pub with_vector: WithVector,

    /// Order the records by a payload field.
    pub order_by: Option<OrderByInterface>,
}

impl Default for ScrollRequestInternal {
    fn default() -> Self {
        ScrollRequestInternal {
            offset: None,
            limit: Some(Self::default_limit()),
            filter: None,
            with_payload: Some(Self::default_with_payload()),
            with_vector: Self::default_with_vector(),
            order_by: None,
        }
    }
}

/// [`LoadProfile`] for opening a read-only shard to serve exactly a scroll with the given filter,
/// `order_by` and `with_payload` selector: no vector components are warmed, and only the field
/// indexes the filter and `order_by` read keep their configured placement.
///
/// Used by edge's `ScrollRequest`.
pub fn scroll_load_profile(
    filter: Option<&Filter>,
    order_by: Option<&OrderByInterface>,
    with_payload: Option<&WithPayloadInterface>,
) -> LoadProfile {
    let order_by_key = order_by.map(|order_by| match order_by {
        OrderByInterface::Key(key) => key,
        OrderByInterface::Struct(order_by) => &order_by.key,
    });
    // The `with_payload` default of a scroll is `true`.
    let with_payload = with_payload.is_none_or(|wp| wp.is_required());

    LoadProfile::for_scroll(filter, order_by_key, with_payload)
}

impl ScrollRequestInternal {
    pub const fn default_limit() -> usize {
        10
    }

    pub const fn default_with_payload() -> WithPayloadInterface {
        WithPayloadInterface::Bool(true)
    }

    pub const fn default_with_vector() -> WithVector {
        WithVector::Bool(false)
    }
}
