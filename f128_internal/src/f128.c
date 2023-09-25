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
  char dat_alt[16];
} __attribute__ ((aligned (16))) Wrapper;

Wrapper strtoflt128_f(const char *s, char **end) {
  return (Wrapper) { strtoflt128(s, end) };
}

#define x_to_f128(x, n) Wrapper n##_to_f128(x a) {    \
    Wrapper d;                                        \
    d.value = (f128) a;                               \
    return d;                                         \
}

#define f128_to_x(x, n) x f128_to_##n(Wrapper a) {   \
  return (x) a.value;\
}

int qtostr(char* s, size_t size, const char *format, Wrapper r) {
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

Wrapper u128_to_f128(unsigned __int128 a) {
    Wrapper d;
    d.value = (f128) a;
    return d;
}

Wrapper i128_to_f128(__int128 a) {
    Wrapper d;
    d.value = (f128) a;
    return d;
}


x_to_f128(int8_t, i8);
x_to_f128(int16_t, i16);
x_to_f128(int32_t, i32);
x_to_f128(int64_t, i64);
x_to_f128(uint8_t, u8);
x_to_f128(uint16_t, u16);
x_to_f128(uint32_t, u32);
x_to_f128(uint64_t, u64);
x_to_f128(float, f32);
x_to_f128(double, f64);
//x_to_f128(__int128, i128)
//x_to_f128(unsigned __int128, u128)
x_to_f128(size_t, usize);
x_to_f128(ssize_t, isize);

 Wrapper f128_add(Wrapper a, Wrapper b){
  Wrapper x;
  x.value = a.value + b.value;
  return x;
}

 Wrapper f128_sub(Wrapper a, Wrapper b){
  Wrapper x;
  x.value = a.value - b.value;
  return x;
}

 Wrapper f128_mul(Wrapper a, Wrapper b){
  Wrapper x;
  x.value = a.value * b.value;
  return x;
}

 Wrapper f128_div(Wrapper a, Wrapper b){
  Wrapper x;
  x.value = a.value / b.value;
  return x;
}

 Wrapper f128_modulo(Wrapper a, Wrapper b){
  Wrapper x;
  x.value = fmodq(a.value, b.value);
  return x;
}

void f128_to_str(Wrapper a, size_t n, char* buf, const char* format) {
  quadmath_snprintf(buf, n, format, a);
}
/* Prototypes for real functions */

 Wrapper acosq_f(Wrapper a) {
  return (Wrapper) { acosq(a.value) };
}
 Wrapper acoshq_f(Wrapper a) {
  return (Wrapper) { acoshq(a.value) };
}
 Wrapper asinq_f (Wrapper a) {
  return (Wrapper) { asinq(a.value) };
}
 Wrapper asinhq_f (Wrapper a) {
   return (Wrapper) { asinhq(a.value) };
}
 Wrapper atanq_f (Wrapper a) {
  return (Wrapper) { atanq(a.value) };
}
 Wrapper atanhq_f (Wrapper a) {
  return (Wrapper) { atanhq(a.value) };
}
 Wrapper atan2q_f (Wrapper a, Wrapper b) {
  return (Wrapper) { atan2q(a.value, b.value) };
}
 Wrapper cbrtq_f (Wrapper a) {
  return (Wrapper) { cbrtq(a.value) };
}
 Wrapper ceilq_f (Wrapper a) {
  return (Wrapper) { ceilq(a.value) };
}
 Wrapper copysignq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { copysignq(a.value, b.value) };
}
 Wrapper coshq_f (Wrapper a) {
  return (Wrapper) { coshq(a.value) };
}
 Wrapper cosq_f (Wrapper a) {
  return (Wrapper) { cosq(a.value) };
}
 Wrapper erfq_f (Wrapper a) {
  return (Wrapper) { erfq(a.value) };
}
 Wrapper erfcq_f (Wrapper a) {
  return (Wrapper) { erfcq(a.value) };
}
 Wrapper expq_f (Wrapper a) {
  return (Wrapper) { expq(a.value) };
}
 Wrapper expm1q_f (Wrapper a) {
  return (Wrapper) { expm1q(a.value) };
}
 Wrapper fabsq_f (Wrapper a) {
  return (Wrapper) { fabsq(a.value) };
}
 Wrapper fdimq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { fdimq(a.value, b.value) };
}
 int finiteq_f (Wrapper a) {
  return finiteq(a.value);
}
 Wrapper floorq_f (Wrapper a) {
  return (Wrapper) { floorq(a.value) };
}
 Wrapper fmaq_f (Wrapper a, Wrapper b, Wrapper c) {
  return (Wrapper) { fmaq(a.value, b.value, c.value) };
}
 Wrapper fmaxq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { fmaxq(a.value, b.value) };
}
 Wrapper fminq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { fminq(a.value, b.value) };
}
 Wrapper fmodq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { fmodq(a.value, b.value) };
}
 Wrapper frexpq_f (Wrapper a, int * b) {
  return (Wrapper) { frexpq(a.value, b) };
}
 Wrapper hypotq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { hypotq(a.value, b.value) };
}
 int isinfq_f (Wrapper a) {
  return isinfq(a.value);
}
 int ilogbq_f (Wrapper a) {
  return ilogbq(a.value);
}
 int isnanq_f (Wrapper a) {
  return isnanq(a.value);
}
 Wrapper j0q_f (Wrapper a) {
  return (Wrapper) { j0q(a.value) };
}
 Wrapper j1q_f (Wrapper a) {
  return (Wrapper) { j1q(a.value) };
}
 Wrapper jnq_f (int a, Wrapper b) {
  return (Wrapper) { jnq(a, b.value) };
}
 Wrapper ldexpq_f (Wrapper a, int b) {
  return (Wrapper) { ldexpq(a.value, b) };
}
 Wrapper lgammaq_f (Wrapper a) {
  return (Wrapper) { lgammaq(a.value) };
}
 long long int llrintq_f (Wrapper a) {
  return llrintq(a.value);
}
 long long int llroundq_f (Wrapper a) {
  return llroundq(a.value);
}

/*
 Wrapper logbq_f (Wrapper a) {
  return (Wrapper) { logbq(a.value) };
}
*/
 Wrapper logq_f (Wrapper a) {
   return (Wrapper) { logq(a.value) };
}
 Wrapper log10q_f (Wrapper a) {
  return (Wrapper) { log10q(a.value) };
}
 Wrapper log2q_f (Wrapper a) {
  return (Wrapper) { log2q(a.value) };
}
 Wrapper log1pq_f (Wrapper a) {
  return (Wrapper) { log1pq(a.value) };
}
 long int lrintq_f (Wrapper a) {
  return lrintq(a.value);
}
 long int lroundq_f (Wrapper a) {
  return lroundq(a.value);
}
 Wrapper modfq_f (Wrapper a, Wrapper * b) {
  return (Wrapper) { modfq(a.value, (f128*) b) };
}
 Wrapper nanq_f (const char * c) {
  return (Wrapper) { nanq(c) };
}
 Wrapper nearbyintq_f (Wrapper a) {
  return (Wrapper) { nearbyintq(a.value) };
}
 Wrapper nextafterq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { nextafterq(a.value, b.value) };
}
 Wrapper powq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { powq(a.value, b.value) };
}
 Wrapper remainderq_f (Wrapper a, Wrapper b) {
  return (Wrapper) { remainderq(a.value, b.value) };
}
 Wrapper remquoq_f (Wrapper a, Wrapper b, int * c) {
  return (Wrapper) { remquoq(a.value, b.value, c) };
}
 Wrapper rintq_f (Wrapper a) {
  return (Wrapper) { rintq(a.value) };
}
 Wrapper roundq_f (Wrapper a) {
  return (Wrapper) { roundq(a.value) };
}
 Wrapper scalblnq_f (Wrapper a, long int b) {
  return (Wrapper) { scalblnq(a.value, b) };
}
 Wrapper scalbnq_f (Wrapper a, int b) {
  return (Wrapper) { scalbnq(a.value, b) };
}
 int signbitq_f (Wrapper a) {
  return signbitq(a.value);
}
 void sincosq_f (Wrapper a, Wrapper * b, Wrapper * c) {
  sincosq(a.value, (f128*) b, (f128*) c);
}
 Wrapper sinhq_f (Wrapper a) {
  return (Wrapper) { sinhq(a.value) };
}
 Wrapper sinq_f (Wrapper a) {
  return (Wrapper) { sinq(a.value) };
}
 Wrapper sqrtq_f (Wrapper a) {
  return (Wrapper) { sqrtq(a.value) };
}
 Wrapper tanq_f (Wrapper a) {
  return (Wrapper) { tanq(a.value) };
}
 Wrapper tanhq_f (Wrapper a) {
  return (Wrapper) { tanhq(a.value) };
}
 Wrapper tgammaq_f (Wrapper a) {
  return (Wrapper) { tgammaq(a.value) };
}
 Wrapper truncq_f (Wrapper a) {
  return (Wrapper) { truncq(a.value) };
}
 Wrapper y0q_f (Wrapper a) {
  return (Wrapper) { y0q(a.value) };
}
 Wrapper y1q_f (Wrapper a) {
  return (Wrapper) { y1q(a.value) };
}
 Wrapper ynq_f(int a, Wrapper b) {
  return (Wrapper) { ynq(a, b.value) };
}

unsigned char gtq(Wrapper a, Wrapper b) {
    return (unsigned char) (a.value > b.value);
}

unsigned char gteq(Wrapper a, Wrapper b) {
    return (unsigned char) (a.value >= b.value);
}
unsigned char ltq(Wrapper a, Wrapper b) {
    return (unsigned char) (a.value < b.value);
}
unsigned char lteq(Wrapper a, Wrapper b) {
    return (unsigned char) (a.value <= b.value);
}
unsigned char eqq(Wrapper a, Wrapper b) {
    return (unsigned char) (a.value == b.value);
}
unsigned char neqq(Wrapper a, Wrapper b) {
    return (unsigned char) (a.value != b.value);
}
