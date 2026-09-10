// ============================================================
// ESCAPE SEQUENCE EXHAUSTIVE DEMO
// ============================================================

// --- Basic escapes ---
std::io::println("=== Basic Escapes ===")
std::io::println("newline\nhere")
std::io::println("tab\there")
std::io::println("carriage\rreturn")
std::io::println("back\\slash")
std::io::println("double\"quote")
std::io::println("single\'quote")
std::io::println("null\0char")

// --- Control character escapes ---
std::io::println("")
std::io::println("=== Control Characters ===")
std::io::println("bell\a")
std::io::println("back\bspace")
std::io::println("form\ffeed")
std::io::println("vertical\vtab")

// --- Hex escapes via \x ---
std::io::println("")
std::io::println("=== Hex Escapes (\\x) ===")
std::io::println("\x41")  // A
std::io::println("\x48\x65\x6c\x6c\x6f")  // Hello

// --- Unicode escapes via \uHHHH ---
std::io::println("")
std::io::println("=== Unicode Fixed (\\uHHHH) ===")
std::io::println("\u0048\u0065\u006c\u006c\u006f")  // Hello

// --- Unicode escapes via \u{} ---
std::io::println("")
std::io::println("=== Unicode Braced (\\u{}) ===")
std::io::println("\u{48}\u{65}\u{6c}\u{6c}\u{6f}")  // Hello
std::io::println("\u{1F600}")  // 😀
std::io::println("\u{1F4A9}")  // 💩
std::io::println("\u{1F680}")  // 🚀

// --- Stars and symbols ---
std::io::println("")
std::io::println("=== Symbols ===")
std::io::println("\u{2605} \u{2605} \u{2605} \u{2606} \u{2606}")
std::io::println("\u{2665}\u{2665}\u{2665} I love rl! \u{2665}\u{2665}\u{2665}")
std::io::println("\u{2713} Pass")
std::io::println("\u{2717} Fail")
std::io::println("\u{26A0} Warning")

// --- Arrows ---
std::io::println("")
std::io::println("=== Arrows ===")
std::io::println("\u{2190} \u{2191} \u{2192} \u{2193}")
std::io::println("\u{2B05}\u{2B06}\u{27A1}\u{2B07}")

// --- Box drawing with \u{} ---
std::io::println("")
std::io::println("=== Box Drawing ===")
std::io::println("\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2557}")
std::io::println("\u{2551}  HELLO WORLD  \u{2551}")
std::io::println("\u{255A}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255D}")

// --- ANSI colors via \e ---
std::io::println("")
std::io::println("=== ANSI Colors ===")
std::io::println("\e[31mRed\e[0m")
std::io::println("\e[32mGreen\e[0m")
std::io::println("\e[33mYellow\e[0m")
std::io::println("\e[34mBlue\e[0m")
std::io::println("\e[35mMagenta\e[0m")
std::io::println("\e[36mCyan\e[0m")

// --- ANSI colors via \x1b ---
std::io::println("")
std::io::println("=== ANSI via \\x1b ===")
std::io::println("\x1b[31mRed\x1b[0m")
std::io::println("\x1b[32mGreen\x1b[0m")

// --- ANSI colors via \u001b ---
std::io::println("")
std::io::println("=== ANSI via \\u001b ===")
std::io::println("\u001b[31mRed\u001b[0m")
std::io::println("\u001b[32mGreen\u001b[0m")

// --- Rainbow ---
std::io::println("")
std::io::println("=== Rainbow ===")
std::io::println("\e[31mR\e[33mE\e[32mD\e[36m \e[34mY\e[35mO\e[31mU\e[33mR\e[0m")

// --- Blinking + inverse ---
std::io::println("")
std::io::println("=== Blink + Inverse ===")
std::io::println("\e[5;31;40m ⚠ BLINKING \e[0m")
std::io::println("\e[7;32m INVERSE \e[0m")

// --- RGB truecolor via \x ---
std::io::println("")
std::io::println("=== RGB Truecolor ===")
std::io::println("\x1b[38;2;255;100;0mOrange\x1b[0m")
std::io::println("\x1b[38;2;0;200;255mCyan\x1b[0m")
std::io::println("\x1b[38;2;200;0;255mPurple\x1b[0m")

// --- 256-color palette via \u001b ---
std::io::println("")
std::io::println("=== 256-Color Palette ===")
std::io::println("\u001b[38;5;196m196\u001b[0m \u001b[38;5;208m208\u001b[0m \u001b[38;5;226m226\u001b[0m \u001b[38;5;34m34\u001b[0m \u001b[38;5;27m27\u001b[0m \u001b[38;5;93m93\u001b[0m")

// --- Bell + flash ---
std::io::println("")
std::io::println("=== Bell ===")
std::io::println("\a\e[5;7m FLASH \e[0m\a")

// --- All three syntaxes side by side ---
std::io::println("")
std::io::println("=== Same char, 3 syntaxes ===")
std::io::println("\e[32m\e  => 0x1B\e[0m")
std::io::println("\x1b[32m\x1b  => 0x1B\x1b[0m")
std::io::println("\u001b[32m\u001b  => 0x1B\u001b[0m")

// --- Final combined demo ---
std::io::println("")
std::io::println("\e[1;36m\u{2713}\e[0m \u{2192} \e[1;33mAll escape sequences work!\e[0m")
std::io::println("\e[32m\u{2605}\e[0m \u{2714} \e[31m\u{2718}\e[0m \u{2716} \e[33m\u{26A0}\e[0m")
