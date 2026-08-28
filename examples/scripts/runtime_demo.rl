get println from std::io

fn divide(int a, int b) -> result[int] {
    if (b == 0) {
        return err(0)
    } else {
        return ok(a / b)
    }
}

println("=== Result type ===")
dec result[int] r1 = ok(42)
dec result[int] r2 = err(1)
println("ok(42) created")
println("err(1) created")

println("=== Function returning result ===")
dec result[int] r3 = divide(10, 2)
println("divide(10, 2) created")
dec result[int] r4 = divide(10, 0)
println("divide(10, 0) created")

println("=== Done ===")
