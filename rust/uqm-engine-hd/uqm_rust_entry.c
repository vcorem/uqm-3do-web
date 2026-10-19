/* Host startup adapter; all game startup logic is translated Rust.
 * Compile with the same Emscripten flags as the game so the compiler lowers
 * main to its required __main_argc_argv entry point. */
extern int uqm_rust_main(int argc, char **argv);
int main(int argc, char **argv) { return uqm_rust_main(argc, argv); }
