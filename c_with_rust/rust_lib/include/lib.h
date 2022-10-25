#ifndef __CUSTOM_RUST_LIB__
#define __CUSTOM_RUST_LIB__

#ifdef __cplusplus
extern "C"
{

// For C++ Notice how this header does not have the same types as in Rust, but the program will still compile.
[[nodiscard]] std::uint64_t power_of(const std::uint64_t base, std::uint64_t exponent);

}
#else

// For C
uint_64 power_of(const uint_8 base, const uint_8 exponent);

#endif


#endif // __CUSTOM_RUST_LIB__
