//! The native-function descriptor embedded in each runtime and the arity
//! metadata it carries.
//!
//! The load-bearing design point: [`NativeHandle::thunk`] is a **thin `fn`
//! pointer**, not a boxed `Rc<dyn Fn>`/`Arc<dyn Fn>`. It is generic over the
//! [`Runtime`] so each runtime monomorphizes the same stdlib body into one
//! concrete function pointer - no per-function heap allocation, no vtable
//! dispatch, and the whole descriptor stays `Copy` so the VM can embed it as a
//! bytecode constant.

use crate::runtime::Runtime;
use crate::signatures::StdFn;
use rl_utils::errors::Error;

/// The number of rl-level arguments a native function accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arity {
    /// Exactly `n` arguments.
    Fixed(usize),
    /// Any number of arguments (variadic, e.g. `print`).
    Variadic,
    /// Between `from` and `to` arguments inclusive.
    Range(usize, usize),
}

impl Arity {
    /// Whether `got` arguments satisfy this arity.
    pub fn accepts(self, got: usize) -> bool {
        match self {
            Arity::Fixed(n) => got == n,
            Arity::Variadic => true,
            Arity::Range(from, to) => (from..=to).contains(&got),
        }
    }

    /// A human-readable description of the expected count, for error messages.
    pub fn describe(self) -> String {
        match self {
            Arity::Fixed(n) => n.to_string(),
            Arity::Variadic => "any number of".to_string(),
            Arity::Range(from, to) => format!("{from} to {to}"),
        }
    }
}

/// The thin function-pointer signature every stdlib function is lowered to.
///
/// `args` is the already-collected argument vector; `span` is the call site,
/// carried as [`Runtime::Span`] (`()` on the VM, a real span on the
/// interpreter, erased to nothing on the VM).
pub type NativeThunk<R> = fn(
    &mut <R as Runtime>::Cx,
    Vec<<R as Runtime>::Value>,
    <R as Runtime>::Span,
) -> Result<<R as Runtime>::Value, Error>;

/// A `'static`, `Copy` native-function descriptor. Embeddable as a VM constant
/// with zero heap allocation.
pub struct NativeHandle<R: Runtime> {
    /// The rl-level name (used for resolution and by-name equality).
    pub name: &'static str,
    /// How many arguments the function accepts.
    pub arity: Arity,
    /// The monomorphized, thin implementation pointer.
    pub thunk: NativeThunk<R>,
    /// Lazily builds the checker signature (kept as a fn pointer so the
    /// descriptor stays `Copy` and the `StdFn`'s heap data is only
    /// materialized when the checker asks for it).
    pub sig: fn() -> StdFn,
}

// Hand-implemented (not derived) so there are no bounds on `R::Value`/`R::Cx`:
// a `NativeHandle` is always trivially copyable regardless of the runtime.
impl<R: Runtime> Clone for NativeHandle<R> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<R: Runtime> Copy for NativeHandle<R> {}

impl<R: Runtime> std::fmt::Debug for NativeHandle<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NativeHandle({})", self.name)
    }
}

// Identity by name, matching the previous `VmNativeFn::eq` semantics. Raw
// `fn`-pointer equality is unreliable across codegen units under LTO, and
// names are unique within any resolved module path.
impl<R: Runtime> PartialEq for NativeHandle<R> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
