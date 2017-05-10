#include <stdio.h>
#include <quadmath.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>
typedef __float128 f128;


extern int quadmath_snprintf (char *s, size_t size, const char *format, ...);

typedef union _Wrapper {
  f128 value;
  unsigned __int128 dat;
} Wrapper;

Wrapper strtoflt128_f(const char *s) {
  char ** sp;
  return (Wrapper) { strtoflt128(s, NULL) };
}

#define x_to_f128(x, n) Wrapper n##_to_f128(x a) {  \
    Wrapper d;\
    d.value = (f128) a;                          \
    return d;\
}

#define f128_to_x(x, n) x f128_to_##n(f128 a) {   \
    return (x) a;\
}

int qtostr(char*s, size_t size, const char *format, Wrapper r) {
  return quadmath_snprintf(s, size, format, r.value);
}

f128_to_x(int8_t, i8)
f128_to_x(int16_t, i16)
f128_to_x(int32_t, i32)
f128_to_x(int64_t, i64)
f128_to_x(uint8_t, u8)
f128_to_x(uint16_t, u16)
f128_to_x(uint32_t, u32)
f128_to_x(uint64_t, u64)
f128_to_x(float, f32)
f128_to_x(double, f64)
f128_to_x(unsigned __int128, u128)
f128_to_x(__int128, i128)
f128_to_x(size_t, usize)
f128_to_x(ssize_t, isize)

x_to_f128(int8_t, i8)
x_to_f128(int16_t, i16)
x_to_f128(int32_t, i32)
x_to_f128(int64_t, i64)
x_to_f128(uint8_t, u8)
x_to_f128(uint16_t, u16)
x_to_f128(uint32_t, u32)
x_to_f128(uint64_t, u64)
x_to_f128(float, f32)
x_to_f128(double, f64)
x_to_f128(__int128, i128)
x_to_f128(unsigned __int128, u128)
x_to_f128(size_t, usize)
x_to_f128(ssize_t, isize)


inline Wrapper f128_add(Wrapper a, Wrapper b){
  Wrapper x;
  x.value = a.value + b.value;
  return x;
}

inline Wrapper f128_sub(Wrapper a, Wrapper b){
  Wrapper x;
  x.value = a.value - b.value;
  return x;
}

inline Wrapper f128_mul(Wrapper a, Wrapper b){
  Wrapper x;
  x.value = a.value * b.value;
  return x;
}

inline Wrapper f128_div(Wrapper a, Wrapper b){
  Wrapper x;
  x.value = a.value / b.value;
  return x;
}

inline Wrapper f128_modulo(Wrapper a, Wrapper b){
  Wrapper x;
  x.value = fmodq(a.value, b.value);
  return x;
}

void f128_to_str(Wrapper a, size_t n, char* buf, const char* format) {
  quadmath_snprintf(buf, n, format, a);
}
/* Prototypes for real functions */

inline Wrapper acosq_f(Wrapper a) {
  return (Wrapper) { acosq(a.value) };
}
inline Wrapper acoshq_f(Wrapper a) {
  return (Wrapper) { acoshq(a.value) };
}
inline Wrapper asinq_f (Wrapper a) {
  return (Wrapper) { asinq(a.value) };
}
inline Wrapper asinhq_f (Wrapper a) {
   return (Wrapper) { asinhq(a.value) };
}
inline Wrapper atanq_f (Wrapper a) {
  return (Wrapper) { atanq(a.value) };
}
inline Wrapper atanhq_f (Wrapper a) {
  return (Wrapper) { atanhq(a.value) };
}
inline Wrapper atan2q_f (Wrapper a, Wrapper b) {
  return (Wrapper) { atan2q(a.value, b.value) };
}
inline Wrapper cbrtq_f (Wrapper a) {
  return (Wrapper) { cbrtq(a.value) };
}
inline Wrapper ceilq_f (Wrapper a) {
  return (Wrapper) { ceilq(a.value) };
}
inline Wrapper copysignq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { copysignq(a.value, b.value) };
}
inline Wrapper coshq_f (Wrapper a) {
  return (Wrapper) { coshq(a.value) };
}
inline Wrapper cosq_f (Wrapper a) {
  return (Wrapper) { cosq(a.value) };
}
inline Wrapper erfq_f (Wrapper a) {
  return (Wrapper) { erfq(a.value) };
}
inline Wrapper erfcq_f (Wrapper a) {
  return (Wrapper) { erfcq(a.value) };
}
inline Wrapper expq_f (Wrapper a) {
  return (Wrapper) { expq(a.value) };
}
inline Wrapper expm1q_f (Wrapper a) {
  return (Wrapper) { expm1q(a.value) };
}
inline Wrapper fabsq_f (Wrapper a) {
  return (Wrapper) { fabsq(a.value) };
}
inline Wrapper fdimq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { fdimq(a.value, b.value) };
}
inline int finiteq_f (Wrapper a) {
  return finiteq(a.value);
}
inline Wrapper floorq_f (Wrapper a) {
  return (Wrapper) { floorq(a.value) };
}
inline Wrapper fmaq_f (Wrapper a, Wrapper b, Wrapper c) {
  return (Wrapper) { fmaq(a.value, b.value, c.value) };
}
inline Wrapper fmaxq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { fmaxq(a.value, b.value) };
}
inline Wrapper fminq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { fminq(a.value, b.value) };
}
inline Wrapper fmodq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { fmodq(a.value, b.value) };
}
inline Wrapper frexpq_f (Wrapper a, int * b) {
  return (Wrapper) { frexpq(a.value, b) };
}
inline Wrapper hypotq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { hypotq(a.value, b.value) };
}
inline int isinfq_f (Wrapper a) {
  return isinfq(a.value);
}
inline int ilogbq_f (Wrapper a) {
  return ilogbq(a.value);
}
inline int isnanq_f (Wrapper a) {
  return isnanq(a.value);
}
inline Wrapper j0q_f (Wrapper a) {
  return (Wrapper) { j0q(a.value) };
}
inline Wrapper j1q_f (Wrapper a) {
  return (Wrapper) { j1q(a.value) };
}
inline Wrapper jnq_f (int a, Wrapper b) {
  return (Wrapper) { jnq(a, b.value) };
}
inline Wrapper ldexpq_f (Wrapper a, int b) {
  return (Wrapper) { ldexpq(a.value, b) };
}
inline Wrapper lgammaq_f (Wrapper a) {
  return (Wrapper) { lgammaq(a.value) };
}
inline long long int llrintq_f (Wrapper a) {
  return llrintq(a.value);
}
inline long long int llroundq_f (Wrapper a) {
  return llroundq(a.value);
}
inline Wrapper logbq_f (Wrapper a) {
  return (Wrapper) { logbq(a.value) };
}
inline Wrapper logq_f (Wrapper a) {
   return (Wrapper) { logq(a.value) };
}
inline Wrapper log10q_f (Wrapper a) {
  return (Wrapper) { log10q(a.value) };
}
inline Wrapper log2q_f (Wrapper a) {
  return (Wrapper) { log2q(a.value) };
}
inline Wrapper log1pq_f (Wrapper a) {
  return (Wrapper) { log1pq(a.value) };
}
inline long int lrintq_f (Wrapper a) {
  return lrintq(a.value);
}
inline long int lroundq_f (Wrapper a) {
  return lroundq(a.value);
}
inline Wrapper modfq_f (Wrapper a, Wrapper * b) {
  return (Wrapper) { modfq(a.value, (f128*) b) };
}
inline Wrapper nanq_f (const char * c) {
  return (Wrapper) { nanq(c) };
}
inline Wrapper nearbyintq_f (Wrapper a) {
  return (Wrapper) { nearbyintq(a.value) };
}
inline Wrapper nextafterq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { nextafterq(a.value, b.value) };
}
inline Wrapper powq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { powq(a.value, b.value) };
}
inline Wrapper remainderq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { remainderq(a.value, b.value) };
}
inline Wrapper remquoq_f (Wrapper a, Wrapper b, int * c) {
  return (Wrapper) { remquoq(a.value, b.value, c) };
}
inline Wrapper rintq_f (Wrapper a) {
  return (Wrapper) { rintq(a.value) };
}
inline Wrapper roundq_f (Wrapper a) {
  return (Wrapper) { roundq(a.value) };
}
inline Wrapper scalblnq_f (Wrapper a, long int b) {
  return (Wrapper) { scalblnq(a.value, b) };
}
inline Wrapper scalbnq_f (Wrapper a, int b) {
  return (Wrapper) { scalbnq(a.value, b) };
}
inline int signbitq_f (Wrapper a) {
  return signbitq(a.value);
}
inline void sincosq_f (Wrapper a, Wrapper * b, Wrapper * c) {
  sincosq(a.value, (f128*) b, (f128*) c);
}
inline Wrapper sinhq_f (Wrapper a) {
  return (Wrapper) { sinhq(a.value) };
}
inline Wrapper sinq_f (Wrapper a) {
  return (Wrapper) { sinq(a.value) };
}
inline Wrapper sqrtq_f (Wrapper a) {
  return (Wrapper) { sqrtq(a.value) };
}
inline Wrapper tanq_f (Wrapper a) {
  return (Wrapper) { tanq(a.value) };
}
inline Wrapper tanhq_f (Wrapper a) {
  return (Wrapper) { tanhq(a.value) };
}
inline Wrapper tgammaq_f (Wrapper a) {
  return (Wrapper) { tgammaq(a.value) };
}
inline Wrapper truncq_f (Wrapper a) {
  return (Wrapper) { truncq(a.value) };
}
inline Wrapper y0q_f (Wrapper a) {
  return (Wrapper) { y0q(a.value) };
}
inline Wrapper y1q_f (Wrapper a) {
  return (Wrapper) { y1q(a.value) };
}
inline Wrapper ynq_f(int a, Wrapper b) {
  return (Wrapper) { ynq(a, b.value) };
}
