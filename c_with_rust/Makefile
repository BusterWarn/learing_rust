FLAGS=-g -Wall -Wextra -Werror -Wpedantic -Wreturn-type \
      -Wparentheses -Wunused -Wundef -Wshadow -Wswitch-default -Wunreachable-code

# Assumes windows compiles with clang and linux with c++

ifdef OS
   RM = rmdir /s/q
else
   ifeq ($(shell uname), Linux)
      RM = rm -rf
   endif
endif

linux_c: main.c rust_lib/target/debug/librust_power_of.so
	mkdir -p out
	gcc --std=c99   main.c   -o out/a.exe ${FLAGS} -iquote rust_lib/include -Isrc -L. -l:rust_lib/target/debug/librust_power_of.so

linux_cpp: main.cpp rust_lib/target/debug/librust_power_of.so
	mkdir -p out
	g++ --std=c++20 main.cpp -o out/a.exe ${FLAGS} -iquote rust_lib/include -L. -l:rust_lib/target/debug/librust_power_of.so

windows_c: main.c rust_lib/target/debug/rust_power_of.dll out
	xcopy rust_lib\target\debug\rust_power_of.dll "out\rust_power_of.dll" /y /b
	clang   -std=c99   main.c   -o out/a.exe ${FLAGS} -iquote rust_lib/include -L. -l:out/rust_power_of.dll

windows_cpp: main.c rust_lib/target/debug/rust_power_of.dll out
	xcopy rust_lib\target\debug\rust_power_of.dll "out\rust_power_of.dll" /y /b
	clang++ -std=c++20 main.cpp -o out/a.exe ${FLAGS} -iquote rust_lib/include -L. -l:out/rust_power_of.dll

out:
	mkdir out

rust_lib/target/debug/rust_power_of.dll:
	cd rust_lib & cargo build --lib

rust_lib/target/debug/librust_power_of.so:
	cd "rust_lib"; cargo build --lib

clean :
	$(RM) out "rust_lib/target"
