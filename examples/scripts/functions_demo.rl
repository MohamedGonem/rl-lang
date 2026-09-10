get println from std::io

dec int x = 10
dec int y = 3

fn max(int a, int b) -> int {
    if (a > b) {
        return a
    } else {
        return b
    }
}

fn my_abs(int n) -> int {
    if (n < 0) {
        return -n
    } else {
        return n
    }
}

fn add(int a, int b) -> int {
    return a + b
}

fn factorial(int n) -> int {
    if (n <= 1) {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}

println("=== Comparison operators ===")
println(max(x, y))
println(max(y, x))
println(my_abs(-42))
println(my_abs(42))

println("=== Arithmetic chain ===")
dec int a = 100
dec int b = 7
println(a + b)
println(a - b)
println(a * b)
println(a / b)

println("=== Boolean logic ===")
dec bool p = true
dec bool q = false
println(p and q)
println(p or q)
println(!q)

println("=== Nested for ===")
dec int total = 0
for [int i = 0, i < 4, i += 1] {
    for [int j = 0, j < 4, j += 1] {
        total = total + 1
    }
}
println(total)

println("=== While countdown ===")
dec int count = 5
while (count > 0) {
    println(count)
    count = count - 1
}

println("=== Functions ===")
println(add(10, 20))
println(factorial(6))

println("=== Casts ===")
dec float pi = 3.14
dec int as_int = pi as int
println(as_int)

println("=== Done ===")
