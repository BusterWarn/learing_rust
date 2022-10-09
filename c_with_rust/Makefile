CXX=clang++
CXXFLAGS=-std=c++20 -g -Wall -Wextra -Werror -Wpedantic -Wreturn-type \
         -Wparentheses -Wunused -Wundef -Wshadow -Wswitch-default -Wunreachable-code

run: compile
	LD_LIBRARY_PATH="${CURDIR}/rust_lib/target/debug" ./out/a.exe

compile: main.cpp
	${CXX} main.cpp -o out/a.exe -iquote rust_lib/include ${CURDIR}
#/LIBPATH:${CURDIR}




# Dynamic library files
run_test_with_dynamic_library: test_with_dynamic_library
	LD_LIBRARY_PATH="${CURDIR}/out" ./out/test_from_dynamic.exe

test_with_dynamic_library:	test/test.cpp library_dynamic
	${CXX} test/test.cpp ${CXXFLAGS} -iquote power_of -B out -L. -lpower_of_dynamic -o out/test_from_dynamic.exe

library_dynamic: power_of/power_of.cpp power_of/power_of.h
	mkdir -p out
	$(CXX) power_of/power_of.cpp ${CXXFLAGS} -fPIC -shared -lc -o out/libpower_of_dynamic.so






clean :
	rm -rf out