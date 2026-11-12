//! Generic DTO codec traits, re-exported from [`cgp_orm`].
//!
//! Input and output types are associated types on the context
//! ([`cgp_orm::HasCreateInputType`], [`cgp_orm::HasUpdateInputType`],
//! [`cgp_orm::HasOutputType`]). This crate never dictates a DTO shape.
//! An application implements these three traits for its own types
//! (see `examples/blog` for distinct Create / Update / Output structs).

pub use cgp_orm::components::service::{
    ApplyUpdateInput, ApplyUpdateInputComponent, CanApplyUpdateInput, CanDecodeCreateInput,
    CanEncodeOutput, DecodeCreateInput, DecodeCreateInputComponent, EncodeOutput,
    EncodeOutputComponent, HasCreateInputType, HasOutputType, HasUpdateInputType,
};
