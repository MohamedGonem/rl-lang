// ============================================================
// TRANSPILER DEMO - features supported by rl-cc
// ============================================================
// Run:  cargo run -- transpile examples/scripts/transpile_demo.rl --compile
// Or:   cargo run -- transpile examples/scripts/transpile_demo.rl --runtime
//       cc -std=c99 -o demo transpile_demo.c rl_runtime.c -I. && ./demo

get println from std::io

// ======================================================================
// SECTION A - compiles and runs end-to-end
// ======================================================================

// --- 1. Variables and arithmetic ---
dec int x = 10
dec int y = 3
dec int sum = x + y
dec int diff = x - y
dec int prod = x * y
dec int quot = x / y
dec int neg = -x

println("=== Arithmetic ===")
println(sum)
println(diff)
println(prod)
println(quot)
println(neg)

// --- 2. Booleans and logic ---
dec bool a = true
dec bool b = false
dec bool cmp_gt = x > y
dec bool cmp_lt = x < y
dec bool cmp_eq = x == y
dec bool cmp_ne = x != y
dec bool cmp_le = x <= y
dec bool cmp_ge = x >= y
dec bool and_res = a and b
dec bool or_res = a or b
dec bool not_res = !a

println("")
println("=== Booleans ===")
println(a)
println(b)
println(cmp_gt)
println(cmp_lt)
println(cmp_eq)
println(cmp_ne)
println(cmp_le)
println(cmp_ge)
println(and_res)
println(or_res)
println(not_res)

// --- 3. Floats ---
dec float pi = 3.14159
dec float half = pi / 2.0

println("")
println("=== Floats ===")
println(pi)
println(half)

// --- 4. Strings and characters ---
dec string greeting = "Hello, world!"
dec string lang = "rl"
dec char ch = 'A'

println("")
println("=== Strings ===")
println(greeting)
println(lang)
println(ch)

// --- 5. Escape sequences ---
println("")
println("=== Escapes ===")
println("tab\there")
println("new\nline")
println("back\\slash")
println("quote\"here")
println("single\'quote")

// --- 6. Constants ---
CONST int MAX = 100
CONST string MSG = "constant string"

println("")
println("=== Constants ===")
println(MAX)
println(MSG)

// --- 7. Null ---
dec int nothing = null

println("")
println("=== Null ===")
println(nothing)

// --- 8. If / else-if / else ---
println("")
println("=== Conditionals ===")
if (x > 5) {
    println("x is big")
} else if (x > 2) {
    println("x is medium")
} else {
    println("x is small")
}

// --- 9. While loop ---
println("")
println("=== While Loop ===")
dec int i = 0
while (i < 3) {
    println(i)
    i = i + 1
}

// --- 10. For loop with break and continue ---
println("")
println("=== For Loop ===")
for [int j = 0, j < 5, j += 1] {
    if (j == 2) {
        continue
    }
    if (j == 4) {
        break
    }
    println(j)
}

// --- 11. Functions ---
fn add(int a, int b) -> int {
    return a + b
}

fn greet(string name) {
    println(name)
}

println("")
println("=== Functions ===")
dec int res = add(100, 23)
println(res)
greet("from a function")

// ======================================================================
// SECTION B - transpiles but C runtime types not yet defined
//            (rl_tuple, rl_array, rl_value, rl_result, RL_TAG_*, len)
// ======================================================================

// --- 12. Cast expressions ---
println("")
println("=== Casts ===")
dec int as_big = 42
dec float from_int = as_big as float
println(from_int)

// --- 13. Tuple literal ---
// dec (int, int, string) t = (1, 2, "three")
// println(t)

// --- 14. Array literal ---
// dec arr[int] nums = [10, 20, 30]
// println(nums[0])
// nums[1] = 99
// println(nums[1])

// --- 15. Record / struct ---
// record Point {
//     int x,
//     int y,
// }
// dec Point p = Point { x: 10, y: 20 }
// println(p.x)
// p.x = 30
// println(p.x)

// --- 16. Enum / tag ---
// tag Color {
//     Red,
//     Green,
//     Blue,
// }
// dec Color c = Color.Red
// println(c)

// --- 17. Match ---
// match (c) {
//     Color.Red => { println("red") }
//     Color.Green => { println("green") }
//     _ => { println("other") }
// }

// --- 18. Ok / Err / Error ---
dec result[int] r = ok(42)
dec result[int] r2 = err(1)

// --- 19. Error propagation ---
fn safe_div(int a, int b) -> result[int] {
    if (b == 0) { return err(0) }
    return ok(a / b)
}
dec result[int] divided = safe_div(10, 2)

// --- Done ---
println("")
println("\e[32mAll end-to-end transpiler features demonstrated!\e[0m")
