//! [`EvalRuntime`] - the [`Runtime`] implementation binding the shared `rl-std`
//! stdlib to the tree-walking interpreter's [`Value`] / [`Evaluator`].
//!
//! Unlike the VM, the interpreter threads a real call [`Span`] into native
//! functions and tracks element/key/value type annotations on its compound
//! values (`items_type` / `key_type`), so the compound constructors keep the
//! annotations the shared stdlib passes.

use crate::evaluator::Evaluator;
use crate::values::{MapKey, Value};
use rl_ast::statements::TypeAnnotation;
use rl_std_core::Runtime;
use rl_utils::errors::Error;
use rl_utils::span::Span;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Zero-sized marker binding the shared stdlib to the interpreter.
pub struct EvalRuntime;

impl Runtime for EvalRuntime {
    type Value = Value;
    type Cx = Evaluator;
    type Span = Span;

    fn error(cx: &Self::Cx, msg: impl Into<String>, span: Self::Span) -> Error {
        cx.err(msg, span)
    }

    fn as_i64(v: &Self::Value) -> Option<i64> {
        if let Value::Integer(x) = v { Some(*x) } else { None }
    }
    fn as_u64(v: &Self::Value) -> Option<u64> {
        if let Value::UInteger(x) = v { Some(*x) } else { None }
    }
    fn as_i32(v: &Self::Value) -> Option<i32> {
        if let Value::SInteger(x) = v { Some(*x) } else { None }
    }
    fn as_u32(v: &Self::Value) -> Option<u32> {
        if let Value::SUInteger(x) = v { Some(*x) } else { None }
    }
    fn as_i16(v: &Self::Value) -> Option<i16> {
        if let Value::BSByte(x) = v { Some(*x) } else { None }
    }
    fn as_u16(v: &Self::Value) -> Option<u16> {
        if let Value::BByte(x) = v { Some(*x) } else { None }
    }
    fn as_i8(v: &Self::Value) -> Option<i8> {
        if let Value::SByte(x) = v { Some(*x) } else { None }
    }
    fn as_u8(v: &Self::Value) -> Option<u8> {
        if let Value::Byte(x) = v { Some(*x) } else { None }
    }
    fn as_f64(v: &Self::Value) -> Option<f64> {
        if let Value::Float(x) = v { Some(*x) } else { None }
    }
    fn as_f32(v: &Self::Value) -> Option<f32> {
        if let Value::SFloat(x) = v { Some(*x) } else { None }
    }
    fn as_bool(v: &Self::Value) -> Option<bool> {
        if let Value::Bool(x) = v { Some(*x) } else { None }
    }
    fn as_char(v: &Self::Value) -> Option<char> {
        if let Value::Char(x) = v { Some(*x) } else { None }
    }
    fn as_str(v: &Self::Value) -> Option<&str> {
        if let Value::String(s) = v { Some(s) } else { None }
    }

    fn type_name(v: &Self::Value) -> &'static str {
        v.type_name()
    }
    fn display(v: &Self::Value) -> String {
        v.to_string()
    }
    fn is_callable(v: &Self::Value) -> bool {
        matches!(v, Value::Function(_))
    }

    fn from_i64(x: i64) -> Self::Value {
        Value::Integer(x)
    }
    fn from_u64(x: u64) -> Self::Value {
        Value::UInteger(x)
    }
    fn from_i32(x: i32) -> Self::Value {
        Value::SInteger(x)
    }
    fn from_u32(x: u32) -> Self::Value {
        Value::SUInteger(x)
    }
    fn from_i16(x: i16) -> Self::Value {
        Value::BSByte(x)
    }
    fn from_u16(x: u16) -> Self::Value {
        Value::BByte(x)
    }
    fn from_i8(x: i8) -> Self::Value {
        Value::SByte(x)
    }
    fn from_u8(x: u8) -> Self::Value {
        Value::Byte(x)
    }
    fn from_f64(x: f64) -> Self::Value {
        Value::Float(x)
    }
    fn from_f32(x: f32) -> Self::Value {
        Value::SFloat(x)
    }
    fn from_bool(x: bool) -> Self::Value {
        Value::Bool(x)
    }
    fn from_char(x: char) -> Self::Value {
        Value::Char(x)
    }
    fn from_string(x: String) -> Self::Value {
        Value::String(x)
    }
    fn null() -> Self::Value {
        Value::Null
    }

    fn ok(v: Self::Value) -> Self::Value {
        Value::Ok(Box::new(v))
    }
    fn err(v: Self::Value) -> Self::Value {
        Value::Err(Box::new(v))
    }
    fn error_value(v: Self::Value) -> Self::Value {
        Value::Error(Box::new(v))
    }

    fn array(items: Vec<Self::Value>, elem: TypeAnnotation) -> Self::Value {
        // Shared stdlib code that builds a computed array passes `Infer` for
        // the element type (it has no static type info). The interpreter tracks
        // `items_type` and its `Value` equality is type-sensitive, so recover a
        // concrete type from the first element, matching the old per-fn logic.
        let items_type = if matches!(elem, TypeAnnotation::Infer) {
            items
                .first()
                .map(|v| Evaluator::infer_type(v, false))
                .unwrap_or(TypeAnnotation::Infer)
        } else {
            elem
        };
        Value::Values { items_type, items }
    }
    fn tuple(items: Vec<Self::Value>) -> Self::Value {
        Value::Tuple(items)
    }
    fn map(
        entries: Vec<(Self::Value, Self::Value)>,
        key: TypeAnnotation,
        val: TypeAnnotation,
    ) -> Self::Value {
        let mut m = HashMap::new();
        for (k, v) in entries {
            if let Some(key) = MapKey::from_value(&k) {
                m.insert(key, v);
            }
        }
        Value::Map {
            key_type: key,
            value_type: val,
            entries: Rc::new(RefCell::new(m)),
        }
    }
    fn set(items: Vec<Self::Value>, elem: TypeAnnotation) -> Self::Value {
        let mut s = HashSet::new();
        for item in items {
            if let Some(key) = MapKey::from_value(&item) {
                s.insert(key);
            }
        }
        Value::Set {
            items_type: elem,
            items: Rc::new(RefCell::new(s)),
        }
    }
    fn as_array(v: &Self::Value) -> Option<(&[Self::Value], TypeAnnotation)> {
        match v {
            Value::Values { items, items_type } => Some((&items[..], items_type.clone())),
            _ => None,
        }
    }
    fn as_tuple(v: &Self::Value) -> Option<&[Self::Value]> {
        match v {
            Value::Tuple(items) => Some(&items[..]),
            _ => None,
        }
    }
    fn as_ok_inner(v: &Self::Value) -> Option<Self::Value> {
        if let Value::Ok(b) = v { Some((**b).clone()) } else { None }
    }
    fn as_err_inner(v: &Self::Value) -> Option<Self::Value> {
        if let Value::Err(b) = v { Some((**b).clone()) } else { None }
    }
    fn as_error_inner(v: &Self::Value) -> Option<Self::Value> {
        if let Value::Error(b) = v { Some((**b).clone()) } else { None }
    }
    fn as_set(v: &Self::Value) -> Option<(Vec<Self::Value>, TypeAnnotation)> {
        match v {
            Value::Set { items, items_type } => Some((
                items.borrow().iter().map(|k| k.clone().into_value()).collect(),
                items_type.clone(),
            )),
            _ => None,
        }
    }
    fn as_map(
        v: &Self::Value,
    ) -> Option<(Vec<(Self::Value, Self::Value)>, TypeAnnotation, TypeAnnotation)> {
        match v {
            Value::Map {
                entries,
                key_type,
                value_type,
            } => Some((
                entries
                    .borrow()
                    .iter()
                    .map(|(k, val)| (k.clone().into_value(), val.clone()))
                    .collect(),
                key_type.clone(),
                value_type.clone(),
            )),
            _ => None,
        }
    }
    fn is_valid_key(v: &Self::Value) -> bool {
        MapKey::from_value(v).is_some()
    }
    fn keys_equal(a: &Self::Value, b: &Self::Value) -> bool {
        match (MapKey::from_value(a), MapKey::from_value(b)) {
            (Some(x), Some(y)) => x == y,
            _ => false,
        }
    }
    fn values_equal(a: &Self::Value, b: &Self::Value) -> bool {
        a == b
    }

    fn set_insert(v: &Self::Value, item: &Self::Value) -> Option<bool> {
        match v {
            Value::Set { items, .. } => {
                let key = MapKey::from_value(item)?;
                Some(items.borrow_mut().insert(key))
            }
            _ => None,
        }
    }
    fn set_remove(v: &Self::Value, item: &Self::Value) -> Option<bool> {
        match v {
            Value::Set { items, .. } => {
                let key = MapKey::from_value(item)?;
                Some(items.borrow_mut().remove(&key))
            }
            _ => None,
        }
    }
    fn set_contains(v: &Self::Value, item: &Self::Value) -> Option<bool> {
        match v {
            Value::Set { items, .. } => {
                let key = MapKey::from_value(item)?;
                Some(items.borrow().contains(&key))
            }
            _ => None,
        }
    }
    fn set_len(v: &Self::Value) -> Option<usize> {
        match v {
            Value::Set { items, .. } => Some(items.borrow().len()),
            _ => None,
        }
    }
    fn set_element_type(v: &Self::Value) -> Option<TypeAnnotation> {
        match v {
            Value::Set { items_type, .. } => Some(items_type.clone()),
            _ => None,
        }
    }

    fn map_insert(v: &Self::Value, key: &Self::Value, value: &Self::Value) -> bool {
        match v {
            Value::Map { entries, .. } => {
                if let Some(key) = MapKey::from_value(key) {
                    entries.borrow_mut().insert(key, value.clone());
                }
                true
            }
            _ => false,
        }
    }
    fn map_get(v: &Self::Value, key: &Self::Value) -> Option<Option<Self::Value>> {
        match v {
            Value::Map { entries, .. } => {
                let key = MapKey::from_value(key)?;
                Some(entries.borrow().get(&key).cloned())
            }
            _ => None,
        }
    }
    fn map_remove(v: &Self::Value, key: &Self::Value) -> Option<Option<Self::Value>> {
        match v {
            Value::Map { entries, .. } => {
                let key = MapKey::from_value(key)?;
                Some(entries.borrow_mut().remove(&key))
            }
            _ => None,
        }
    }
    fn map_contains(v: &Self::Value, key: &Self::Value) -> Option<bool> {
        match v {
            Value::Map { entries, .. } => {
                let key = MapKey::from_value(key)?;
                Some(entries.borrow().contains_key(&key))
            }
            _ => None,
        }
    }
    fn map_len(v: &Self::Value) -> Option<usize> {
        match v {
            Value::Map { entries, .. } => Some(entries.borrow().len()),
            _ => None,
        }
    }
    fn map_key_value_types(v: &Self::Value) -> Option<(TypeAnnotation, TypeAnnotation)> {
        match v {
            Value::Map {
                key_type,
                value_type,
                ..
            } => Some((key_type.clone(), value_type.clone())),
            _ => None,
        }
    }
    fn map_clear(v: &Self::Value) -> bool {
        match v {
            Value::Map { entries, .. } => {
                entries.borrow_mut().clear();
                true
            }
            _ => false,
        }
    }
    fn map_for_each<F: FnMut(Self::Value, Self::Value)>(v: &Self::Value, mut f: F) -> bool {
        match v {
            Value::Map { entries, .. } => {
                for (k, val) in entries.borrow().iter() {
                    f(k.clone().into_value(), val.clone());
                }
                true
            }
            _ => false,
        }
    }

    fn value_type(v: &Self::Value) -> TypeAnnotation {
        Evaluator::infer_type(v, false)
    }
    fn types_compatible(actual: &TypeAnnotation, expected: &TypeAnnotation) -> bool {
        Evaluator::types_compatible(actual, expected)
    }

    fn call_value(
        cx: &mut Self::Cx,
        callee: &Self::Value,
        args: &[Self::Value],
        span: Self::Span,
    ) -> Result<Self::Value, Error> {
        cx.call_value(callee.clone(), args.to_vec(), span)
    }
    fn callable_return_type(v: &Self::Value) -> Option<TypeAnnotation> {
        match v {
            Value::Function(data) => data.return_type.clone(),
            _ => None,
        }
    }

    fn rng(cx: &mut Self::Cx) -> &mut rl_std_core::Xoshiro256 {
        &mut cx.rng
    }
    fn output_buffer(cx: &mut Self::Cx) -> &mut Option<String> {
        &mut cx.output_buffer
    }
    fn user_args_offset(cx: &Self::Cx) -> usize {
        cx.user_args_offset
    }
    fn as_handle(v: &Self::Value, kind: rl_ast::statements::HandleKind) -> Option<u64> {
        match v {
            Value::Handle { kind: k, id } if *k == kind => Some(*id),
            _ => None,
        }
    }
    fn make_handle(kind: rl_ast::statements::HandleKind, id: u64) -> Self::Value {
        Value::Handle { kind, id }
    }
}

impl rl_std::net::NetStore for EvalRuntime {
    fn net_insert(cx: &mut Evaluator, h: rl_std::net::NetHandle) -> u64 {
        let id = cx.net_next_handle;
        cx.net_next_handle += 1;
        cx.net_handles.insert(id, h);
        id
    }
    fn net_get(cx: &Evaluator, id: u64) -> Option<&rl_std::net::NetHandle> {
        cx.net_handles.get(&id)
    }
    fn net_get_mut(cx: &mut Evaluator, id: u64) -> Option<&mut rl_std::net::NetHandle> {
        cx.net_handles.get_mut(&id)
    }
    fn net_remove(cx: &mut Evaluator, id: u64) -> Option<rl_std::net::NetHandle> {
        cx.net_handles.remove(&id)
    }
}

impl rl_std::c::CStore for EvalRuntime {
    fn c_insert(cx: &mut Evaluator, h: rl_std::c::CHandle) -> u64 {
        let id = cx.c_next_handle;
        cx.c_next_handle += 1;
        cx.c_handles.insert(id, h);
        id
    }
    fn c_get(cx: &Evaluator, id: u64) -> Option<&rl_std::c::CHandle> {
        cx.c_handles.get(&id)
    }
    fn c_get_mut(cx: &mut Evaluator, id: u64) -> Option<&mut rl_std::c::CHandle> {
        cx.c_handles.get_mut(&id)
    }
    fn c_remove(cx: &mut Evaluator, id: u64) -> Option<rl_std::c::CHandle> {
        cx.c_handles.remove(&id)
    }
}

impl rl_std::http::HttpStore for EvalRuntime {
    fn http_insert(cx: &mut Evaluator, h: rl_std::http::HttpHandle) -> u64 {
        let id = cx.http_next_handle;
        cx.http_next_handle += 1;
        cx.http_handles.insert(id, h);
        id
    }
    fn http_get(cx: &Evaluator, id: u64) -> Option<&rl_std::http::HttpHandle> {
        cx.http_handles.get(&id)
    }
    fn http_get_mut(cx: &mut Evaluator, id: u64) -> Option<&mut rl_std::http::HttpHandle> {
        cx.http_handles.get_mut(&id)
    }
    fn http_remove(cx: &mut Evaluator, id: u64) -> Option<rl_std::http::HttpHandle> {
        cx.http_handles.remove(&id)
    }
}

impl rl_std::audio::AudioStore for EvalRuntime {
    fn audio_insert(cx: &mut Evaluator, h: rl_std::audio::AudioHandle) -> u64 {
        let id = cx.audio_next_handle;
        cx.audio_next_handle += 1;
        cx.audio_handles.insert(id, h);
        id
    }
    fn audio_get(cx: &Evaluator, id: u64) -> Option<&rl_std::audio::AudioHandle> {
        cx.audio_handles.get(&id)
    }
    fn audio_get_mut(cx: &mut Evaluator, id: u64) -> Option<&mut rl_std::audio::AudioHandle> {
        cx.audio_handles.get_mut(&id)
    }
    fn audio_remove(cx: &mut Evaluator, id: u64) -> Option<rl_std::audio::AudioHandle> {
        cx.audio_handles.remove(&id)
    }
    fn audio_output_device(cx: &mut Evaluator) -> &mut Option<String> {
        &mut cx.audio_output_device
    }
    fn audio_master_volume(cx: &mut Evaluator) -> &mut f32 {
        &mut cx.audio_master_volume
    }
    fn audio_handles_values<'a>(
        cx: &'a Evaluator,
    ) -> Box<dyn Iterator<Item = &'a rl_std::audio::AudioHandle> + 'a> {
        Box::new(cx.audio_handles.values())
    }
}

impl rl_std::gui::GuiStore for EvalRuntime {
    fn gui_handles(
        cx: &mut Evaluator,
    ) -> &mut std::collections::HashMap<u64, rl_std::gui::GuiHandle<Value>> {
        &mut cx.gui_handles
    }
    fn gui_handles_ref(
        cx: &Evaluator,
    ) -> &std::collections::HashMap<u64, rl_std::gui::GuiHandle<Value>> {
        &cx.gui_handles
    }
    fn gui_next_handle(cx: &mut Evaluator) -> &mut u64 {
        &mut cx.gui_next_handle
    }
    fn gui_quit_requested(cx: &mut Evaluator) -> &mut bool {
        &mut cx.gui_quit_requested
    }
}
