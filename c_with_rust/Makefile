FLAGS=-g -Wall -Wextra -Werror -Wpedantic -Wreturn-type \
      -Wparentheses -Wunused -Wundef -Wshadow -Wswitch-default -Wunreachable-code

cpp: main.cpp rust_lib/target/debug/librust_power_of.so
	mkdir -p out
	g++ --std=c++20 main.cpp ${FLAGS} -o out/a.exe -iquote rust_lib/include -L. -l:rust_lib/target/debug/librust_power_of.so

c: main.c rust_lib/target/debug/librust_power_of.so
	mkdir -p out
	gcc --std=c99 -o out/a.exe ${FLAGS} main.c -iquote rust_lib/include -Isrc -L. -l:rust_lib/target/debug/librust_power_of.so

rust_lib/target/debug/librust_power_of.so:
	cd "rust_lib"; \
	cargo build --lib

clean :
	rm -rf out rust_lib/target