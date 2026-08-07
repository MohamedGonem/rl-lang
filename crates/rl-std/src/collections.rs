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
//! matching the old `vok!` / `verr!` bodies. The hot mutating/query operations
//! (`set_add`, `set_contains`, `set_len`, `map_merge`, `map_get`, ...) use the
//! `Runtime` in-place accessors (O(1) hash ops, mutating the shared set/map),
//! restoring the behaviour of the old per-runtime implementations. The
//! whole-container accessors (`as_set` / `as_map`) are only used where a
//! function genuinely needs every element (`set_to_array`, `map_to_array`,
//! `map_keys`, `map_values`).

use rl_ast::statements::TypeAnnotation;
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
use std::rc::Rc;

// ---- sets -----------------------------------------------------------------

#[native_fn(module = "collections", sig(set[T], T -> result[set[T]]))]
pub fn set_add<R: Runtime>(set: R::Value, value: R::Value) -> R::Value {
    let Some(elem) = R::set_element_type(&set) else {
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
    R::set_insert(&set, &value);
    R::ok(set)
}

#[native_fn(module = "collections", sig(set[T], T -> result[set[T]]))]
pub fn set_remove<R: Runtime>(set: R::Value, value: R::Value) -> R::Value {
    if !R::is_valid_key(&value) {
        return R::err(R::from_string(format!(
            "set_remove: cannot remove {} from a set",
            R::type_name(&value)
        )));
    }
    match R::set_remove(&set, &value) {
        Some(_) => R::ok(set),
        None => R::err(R::from_string(format!(
            "set_remove: accepts only sets, found {}",
            R::type_name(&set)
        ))),
    }
}

#[native_fn(module = "collections", sig(set[T], T -> result[bool]))]
pub fn set_contains<R: Runtime>(set: R::Value, value: R::Value) -> R::Value {
    if !R::is_valid_key(&value) {
        return R::ok(R::from_bool(false));
    }
    match R::set_contains(&set, &value) {
        Some(found) => R::ok(R::from_bool(found)),
        None => R::err(R::from_string(format!(
            "set_contains: accepts only sets, found {}",
            R::type_name(&set)
        ))),
    }
}

#[native_fn(module = "collections", sig(set[T] -> result[int]))]
pub fn set_len<R: Runtime>(set: R::Value) -> R::Value {
    match R::set_len(&set) {
        Some(len) => R::ok(R::from_i64(len as i64)),
        None => R::err(R::from_string(format!(
            "set_len: accepts only sets, found {}",
            R::type_name(&set)
        ))),
    }
}

#[native_fn(module = "collections", sig(set[T] -> result[bool]))]
pub fn set_is_empty<R: Runtime>(set: R::Value) -> R::Value {
    match R::set_len(&set) {
        Some(len) => R::ok(R::from_bool(len == 0)),
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
    if !R::is_valid_key(&key) {
        return R::ok(R::from_bool(false));
    }
    match R::map_contains(&map, &key) {
        Some(found) => R::ok(R::from_bool(found)),
        None => R::err(R::from_string(format!(
            "map_contains: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V], K -> result[map[K, V]]))]
pub fn map_remove<R: Runtime>(map: R::Value, key: R::Value) -> R::Value {
    if !R::is_valid_key(&key) {
        return R::err(R::from_string(format!(
            "map_remove: cannot remove {} from a map",
            R::type_name(&key)
        )));
    }
    match R::map_remove(&map, &key) {
        Some(_) => R::ok(map),
        None => R::err(R::from_string(format!(
            "map_remove: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[int]))]
pub fn map_len<R: Runtime>(map: R::Value) -> R::Value {
    match R::map_len(&map) {
        Some(len) => R::ok(R::from_i64(len as i64)),
        None => R::err(R::from_string(format!(
            "map_len: accepts only maps, found {}",
            R::type_name(&map)
        ))),
    }
}

#[native_fn(module = "collections", sig(map[K, V] -> result[bool]))]
pub fn map_is_empty<R: Runtime>(map: R::Value) -> R::Value {
    match R::map_len(&map) {
        Some(len) => R::ok(R::from_bool(len == 0)),
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
    if !R::is_valid_key(&key) {
        return R::err(R::from_string(format!(
            "map_get: cannot use {} as a map key",
            R::type_name(&key)
        )));
    }
    match R::map_get(&map, &key) {
        Some(Some(value)) => R::ok(value),
        Some(None) => R::err(R::from_string(format!(
            "map_get: key {} not found in map",
            R::display(&key)
        ))),
        None => R::err(R::from_string(format!(
            "map_get: accepts only maps, found {}",
            R::type_name(&map)
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
    if R::map_clear(&map) {
        R::ok(map)
    } else {
        R::err(R::from_string(format!(
            "map_clear: accepts only maps, found {}",
            R::type_name(&map)
        )))
    }
}

#[native_fn(module = "collections", sig(map[K, V], map[K, V] -> result[map[K, V]]))]
pub fn map_merge<R: Runtime>(map1: R::Value, map2: R::Value) -> R::Value {
    if !R::map_for_each(&map2, |k, v| {
        R::map_insert(&map1, &k, &v);
    }) {
        return R::err(R::from_string(format!(
            "map_merge: accepts only maps, found {}",
            R::type_name(&map1)
        )));
    }
    R::ok(map1)
}

rl_std_core::native_module!("collections";
    funcs: [
        set_add, set_remove, set_contains, set_len, set_is_empty, set_to_array,
        map_contains, map_remove, map_len, map_is_empty, map_to_array, map_get,
        map_keys, map_values, map_clear, map_merge,
    ],
);
