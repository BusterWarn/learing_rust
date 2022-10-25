#ifndef __CUSTOM_RUST_LIB__
#define __CUSTOM_RUST_LIB__

#ifdef __cplusplus
extern "C"
{

// For C++
[[nodiscard]] std::uint64_t power_of(const std::uint64_t base, std::uint64_t exponent);

}
#else

// For C
unsigned int power_of(const unsigned long base, const unsigned short exponent);

#endif


#endif // __CUSTOM_RUST_LIB__
