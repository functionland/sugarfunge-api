// SBP-M1 review: add doc comments to all types and fields, which can be used for generation API docs
#[subxt::subxt(
    runtime_metadata_path = "sugarfunge_metadata.scale",
    derive_for_type(
        type = "frame_support::traits::tokens::misc::BalanceStatus",
        derive = "serde::Serialize"
    ),
    derive_for_type(type = "pallet_balances::pallet::Event", derive = "serde::Serialize"),
    derive_for_type(type = "sugarfunge_asset::pallet::Event", derive = "serde::Serialize"),
    derive_for_type(type = "sugarfunge_bag::pallet::Event", derive = "serde::Serialize")
)]
pub mod sugarfunge {}
pub mod account;
pub mod asset;
pub mod bag;
pub mod bundle;
pub mod challenge;
pub mod contract;
pub mod fula;
pub mod market;
pub mod pool;
pub mod primitives;
pub mod validator;

// SBP-M1 review: this crate uses subxt as a dependency and is itself used by the proof-engine.
// SBP-M1 review: A recommendation is to add a simple API to this crate which facilitates signing of extrinsics on the 'client', rather than within the 'sugarfunge-api' via seeds being passed around.
