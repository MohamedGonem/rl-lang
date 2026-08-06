//! `std::collections` - functions for working with `set[T]` and `map[K, V]`.
//!
//! The rl module name is `collections`; the Rust module is `collections`.
//! Ported once from the former per-runtime `stdlib/collections/*.rs` copies.
//!
//! These functions are value-polymorphic over the element/key/value types, so
//! they take raw `R::Value` arguments and use dedicated `Runtime` accessors to
//! read and rebuild sets/maps. Their explicit `sig(...)` overloads mirror
//! `rl-commons/src/stdlib_signatures/collections.rs` (`set[T]`, `map[K, V]`).
//!
//! Every function returns a language `result[T]` value (`ok(..)` / `err(..)`),
//! matching the old `vok!` / `verr!` bodies. Because the `Runtime` abstraction
//! cannot mutate a set/map in place, the mutating functions (`set_add`,
//! `set_remove`, `map_remove`, `map_clear`, `map_merge`) read the current
//! contents and rebuild a fresh `set`/`map` via `R::set` / `R::map`.

use rl_ast::statements::TypeAnnotation;
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
use std::rc::Rc;

// ---- sets -----------------------------------------------------------------

#[native_fn(module = "collections", sig(set[T], T -> result[set[T]]))]
pub fn set_add<R: Runtime>(set: R::Value, value: R::Value) -> R::Value {
    let Some((mut items, elem)) = R::as_set(&set) else {
        return R::err(R::from_string(format!(
            "set_add: accepts only sets, found {}",
            R::type_name(&set)
        )));
    };
    // Interpreter-only element-type check (a no-op on the VM, whose sets carry
    // no tracked element type).
    let val_type = R::value_type(&value);
    if !R::types_compatible(&val_type, &elem) {
        return R::err(R::from_string(format!(
            "set_add: type mismatch: set expects {elem:?}, cannot add {val_type:?}"
        )));
    }
    if !R::is_valid_key(&value) {
        return R::err(R::from_string(format!(
            "set_add: cannot add {} to a set",
            R::type_name(&value)
        )));
    }
    items.push(value);
    R::ok(R::set(items, elem))
}

#[native_fn(module = "collections", sig(set[T], T -> result[set[T]]))]
pub fn set_remove<R: Runtime>(set: R::Value, value: R::Value) -> R::Value {
    let Some((items, elem)) = R::as_set(&set) else {
        return R::err(R::from_string(format!(
            "set_remove: accepts only sets, found {}",
            R::type_name(&set)
        )));
    };
    if !R::is_valid_key(&value) {
        return R::err(R::from_string(format!(
            "set_remove: cannot remove {} from a set",
            R::type_name(&value)
        )));
    }
    let items: Vec<R::Value> = items
        .into_iter()
        .filter(|v| !R::keys_equal(v, &value))
        .collect();
    R::ok(R::set(items, elem))
}

#[native_fn(module = "collections", sig(set[T], T -> result[bool]))]
pub fn set_contains<R: Runtime>(set: R::Value, value: R::Value) -> R::Value {
    let Some((items, _)) = R::as_set(&set) else {
        return R::err(R::from_string(format!(
            "set_contains: accepts only sets, found {}",
            R::type_name(&set)
        )));
    };
    if !R::is_valid_key(&value) {
        return R::ok(R::from_bool(false));
    }
    R::ok(R::from_bool(items.iter().any(|v| R::keys_equal(v, &value))))
}

#[native_fn(module = "collections", sig(set[T] -> result[int]))]
pub fn set_len<R: Runtime>(set: R::Value) -> R::Value {
    match R::as_set(&set) {
        Some((items, _)) => R::ok(R::from_i64(items.len() as i64)),
        None => R::err(R::from_string(format!(
            "set_len: accepts only sets, found {}",
            R::type_name(&set)
        ))),
    }
}

#[native_fn(module = "collections", sig(set[T] -> result[bool]))]
pub fn set_is_empty<R: Runtime>(set: R::Value) -> R::Value {
    match R::as_set(&set) {
        Some((items, _)) => R::ok(R::from_bool(items.is_empty())),
        None => R::err(R::from_string(format!(
            "set_is_empty: accepts only sets, found {}",
            R::type_name(&set)
        ))),
    }
}

#[native_fn(module = "collections", sig(set[T] -> result[array[T]]))]
pub fn set_to_array<R: Runtime>(set: R::Value) -> R::Value {
    match R::as_set(&set) {
        Some((items, elem)) => R::ok(R::array(items, elem)),
        None => R::err(R::from_string(format!(
            "set_to_array: accepts only sets, found {}",
            R::type_name(&set)
        ))),
    }
}

// ---- maps -----------------------------------------------------------------

#[native_fn(module = "collections", sig(map[K, V], K -> result[bool]))]
pub fn map_contains<R: Runtime>(map: R::Value, key: R::Value) -> R::Value {
    let Some((entries, ..)) = R::as_map(&map) else {
        return R::err(R::from_string(format!(
            "map_contains: accepts only maps, found {}",
            R::type_name(&map)
        )));
    };
    if !R::is_valid_key(&key) {
        return R::ok(R::from_bool(false));
    }
    R::ok(R::from_bool(
        entries.iter().any(|(k, _)| R::keys_equal(k, &key)),
    ))
}

#[native_fn(module = "collections", sig(map[K, V], K -> result[map[K, V]]))]
pub fn map_remove<R: Runtime>(map: R::Value, key: R::Value) -> R::Value {
    let Some((entries, key_ty, val_ty)) = R::as_map(&map) else {
        return R::err(R::from_string(format!(
            "map_remove: accepts only maps, found {}",
            R::type_name(&map)
        )));
    };
    if !R::is_valid_key(&key) {
        return R::err(R::from_string(format!(
            "map_remove: cannot remove {} from a map",
            R::type_name(&key)
        )));
    }
    let entries: Vec<(R::Value, R::Value)> = entries
        .into_iter()
        .filter(|(k, _)| !R::keys_equal(k, &key))
        .collect();
    R::ok(R::map(entries, key_ty, val_ty))
}

#[native_fn(module = "collections", sig(map[K, V] -> result[int]))]
pub fn map_len<R: Runtime>(map: R::Value) -> R::Value {
    match R::as_map(&map) {
        Some((entries, ..)) => R::ok(R::from_i64(entries.len() as i64)),
        None => R::err(R::from_string(format!(
            "map_len: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[bool]))]
pub fn map_is_empty<R: Runtime>(map: R::Value) -> R::Value {
    match R::as_map(&map) {
        Some((entries, ..)) => R::ok(R::from_bool(entries.is_empty())),
        None => R::err(R::from_string(format!(
            "map_is_empty: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[array[tuple[K, V]]]))]
pub fn map_to_array<R: Runtime>(map: R::Value) -> R::Value {
    match R::as_map(&map) {
        Some((entries, key_ty, val_ty)) => {
            let items: Vec<R::Value> = entries
                .into_iter()
                .map(|(k, v)| R::tuple(vec![k, v]))
                .collect();
            let elem = TypeAnnotation::Tuple(Rc::new(vec![key_ty, val_ty]));
            R::ok(R::array(items, elem))
        }
        None => R::err(R::from_string(format!(
            "map_to_array: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V], K -> result[V]))]
pub fn map_get<R: Runtime>(map: R::Value, key: R::Value) -> R::Value {
    let Some((entries, ..)) = R::as_map(&map) else {
        return R::err(R::from_string(format!(
            "map_get: accepts only maps, found {}",
            R::type_name(&map)
        )));
    };
    if !R::is_valid_key(&key) {
        return R::err(R::from_string(format!(
            "map_get: cannot use {} as a map key",
            R::type_name(&key)
        )));
    }
    match entries.into_iter().find(|(k, _)| R::keys_equal(k, &key)) {
        Some((_, value)) => R::ok(value),
        None => R::err(R::from_string(format!(
            "map_get: key {} not found in map",
            R::display(&key)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[array[K]]))]
pub fn map_keys<R: Runtime>(map: R::Value) -> R::Value {
    match R::as_map(&map) {
        Some((entries, key_ty, _)) => {
            let items: Vec<R::Value> = entries.into_iter().map(|(k, _)| k).collect();
            R::ok(R::array(items, key_ty))
        }
        None => R::err(R::from_string(format!(
            "map_keys: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[array[V]]))]
pub fn map_values<R: Runtime>(map: R::Value) -> R::Value {
    match R::as_map(&map) {
        Some((entries, _, val_ty)) => {
            let items: Vec<R::Value> = entries.into_iter().map(|(_, v)| v).collect();
            R::ok(R::array(items, val_ty))
        }
        None => R::err(R::from_string(format!(
            "map_values: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[map[K, V]]))]
pub fn map_clear<R: Runtime>(map: R::Value) -> R::Value {
    match R::as_map(&map) {
        Some((_, key_ty, val_ty)) => R::ok(R::map(Vec::new(), key_ty, val_ty)),
        None => R::err(R::from_string(format!(
            "map_clear: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V], map[K, V] -> result[map[K, V]]))]
pub fn map_merge<R: Runtime>(map1: R::Value, map2: R::Value) -> R::Value {
    let Some((mut entries, key_ty, val_ty)) = R::as_map(&map1) else {
        return R::err(R::from_string(format!(
            "map_merge: accepts only maps, found {}",
            R::type_name(&map1)
        )));
    };
    let Some((entries2, ..)) = R::as_map(&map2) else {
        return R::err(R::from_string(format!(
            "map_merge: accepts only maps, found {}",
            R::type_name(&map2)
        )));
    };
    for (k, v) in entries2 {
        entries.retain(|(ek, _)| !R::keys_equal(ek, &k));
        entries.push((k, v));
    }
    R::ok(R::map(entries, key_ty, val_ty))
}

rl_std_core::native_module!("collections";
    funcs: [
        set_add, set_remove, set_contains, set_len, set_is_empty, set_to_array,
        map_contains, map_remove, map_len, map_is_empty, map_to_array, map_get,
        map_keys, map_values, map_clear, map_merge,
    ],
);
