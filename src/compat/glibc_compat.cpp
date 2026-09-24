#if defined(__linux__) && (defined(__x86_64__) || defined(__i386__))
#include <string>
#include <cstdlib>

extern "C" {

// GLIBC 2.38 added ISO C23 strto* functions which are called by newer prebuilt ONNX Runtime.
// On older GLIBC (e.g. GLIBC 2.35 on Ubuntu 22.04), provide backwards-compatible wrappers.
long long __isoc23_strtoll(const char *nptr, char **endptr, int base) {
    return strtoll(nptr, endptr, base);
}

long __isoc23_strtol(const char *nptr, char **endptr, int base) {
    return strtol(nptr, endptr, base);
}

unsigned long long __isoc23_strtoull(const char *nptr, char **endptr, int base) {
    return strtoull(nptr, endptr, base);
}

// In GCC 13 / libstdc++ 13+, _M_replace_cold handles cold replacement paths.
// In GCC 11 / libstdc++ 11 (Ubuntu 22.04), route to std::basic_string::replace.
std::__cxx11::basic_string<char>& _ZNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEE15_M_replace_coldEPcmPKcmm(
    std::__cxx11::basic_string<char>* self, char* p, unsigned long len1, const char* s, unsigned long len2, unsigned long
) {
    return self->replace(p - self->data(), len1, s, len2);
}

std::__cxx11::basic_string<wchar_t>& _ZNSt7__cxx1112basic_stringIwSt11char_traitsIwESaIwEE15_M_replace_coldEPwmPKwmm(
    std::__cxx11::basic_string<wchar_t>* self, wchar_t* p, unsigned long len1, const wchar_t* s, unsigned long len2, unsigned long
) {
    return self->replace(p - self->data(), len1, s, len2);
}

}
#endif
