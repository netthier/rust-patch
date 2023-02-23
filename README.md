# rust-patch
Patch structs with other structs

[![Build status](https://github.com/netthier/rust-patch/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/netthier/rust-patch/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/rust-patch)](https://crates.io/crates/rust-patch)
[![Documentation](https://docs.rs/rust-patch/badge.svg)](https://docs.rs/rust-patch)

`rust-patch` allows you to avoid boilerplate code when implementing partial updates of Rust structs.  
Simply define a patch struct containing a subset of your fields, derive the `Patch` trait,
and specify the original struct using the `#[patch]` attribute.  
Fields of a patch struct may either be of the same type `T` as in the original struct or `Option<T>`.  
In the latter case, the field to be patched will be left unchanged if the corresponding field in the patch is `None`

This crate is `no_std` compatible.

## Container attributes
### `#[patch = "..."]`
Set target struct to be patched
```rust
use rust_patch::Patch;
struct Item { data: u32 }

#[derive(Patch)]
#[patch = "Item"]
struct ItemPatch { data: Option<u32> }
```
## Field attributes
### `#[patch(direct)]`
By default, any fields in the patch of type `Option<T>` will be applied as such:
```rust ignore
if let Some(val) = patch.field {
    target.field = val;
} 
```
In some cases, e.g. when the field in the target struct is also an `Option`, this behavior is undesirable. 
The `direct` attribute makes it so that the field is treated like any other `T`, meaning it will be applied like this:
```rust ignore
target.field = patch.field;
```

### `#[patch(as_option)]`
This attribute also primarily changes the handling of `Option` types.
Unlike with `direct` however, the original value will not be overwritten if the patch value is `None`.
```rust ignore
if patch.field.is_some() {
    target.field = patch.field;
}
```

## Example
```rust
use rust_patch::Patch;
use serde::Deserialize;

#[derive(PartialEq, Debug)]
struct User {
    id: String,
    display_name: String,
    email: String,
}

#[derive(Deserialize, Patch)]
#[patch = "User"]
struct UserPatch {
    display_name: Option<String>,
    email: Option<String>,
}

let user = User {
    id: "6bf25b70-bffa-49e0-905b-2d2e608e3abd".to_string(),
    display_name: "Max Mustermann".to_string(),
    email: "max.mustermann@example.org".to_string(),
};

let raw_patch = r#"{
    "id": "some invalid id",
    "email": "max.mustermann@example.com"
}"#;

let patch: UserPatch = serde_json::from_str(raw_patch).unwrap();
let patched_user = patch.apply(user);

// Since `id` is not part of our `UserPatch` struct it stays unchanged
assert_eq! {
    patched_user,
    User {
        id: "6bf25b70-bffa-49e0-905b-2d2e608e3abd".to_string(),
        display_name: "Max Mustermann".to_string(),
        email: "max.mustermann@example.com".to_string()
    }
};
```