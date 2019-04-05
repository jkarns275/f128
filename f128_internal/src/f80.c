#include <math.h>
#include <float.h>
#include <stdint.h>

#define x_to_ld(type, rname) LD rname##_to_ld(type x) {               \
    return (LD) { .value = (long double) x };                         \
}
#define ld_to_x(type, rname) type ld_to_##rname(LD x) {  \
    return (type) x.value;\
}

typedef union _LD {
  char dummy[12];
  long double value;
  unsigned __int128 u;
  signed __int128 i;
} LD;

ld_to_x(int8_t, i8)
ld_to_x(int16_t, i16)
ld_to_x(int32_t, i32)
ld_to_x(int64_t, i64)
ld_to_x(uint8_t, u8)
ld_to_x(uint16_t, u16)
ld_to_x(uint32_t, u32)
ld_to_x(uint64_t, u64)
ld_to_x(float, f32)
ld_to_x(double, f64)
ld_to_x(unsigned __int128, u128)
ld_to_x(__int128, i128)
ld_to_x(size_t, usize)
ld_to_x(ssize_t, isize)

x_to_ld(int8_t, i8)
x_to_ld(int16_t, i16)
x_to_ld(int32_t, i32)
x_to_ld(int64_t, i64)
x_to_ld(uint8_t, u8)
x_to_ld(uint16_t, u16)
x_to_ld(uint32_t, u32)
x_to_ld(uint64_t, u64)
x_to_ld(float, f32)
x_to_ld(double, f64)
x_to_ld(__int128, i128)
x_to_ld(unsigned __int128, u128)
x_to_ld(size_t, usize)
x_to_ld(ssize_t, isize)


inline LD ld_add(LD a, LD b){
  LD x;
  x.value = a.value + b.value;
  return x;
}

inline LD ld_sub(LD a, LD b){
  LD x;
  x.value = a.value - b.value;
  return x;
}

inline LD ld_mul(LD a, LD b){
  LD x;
  x.value = a.value * b.value;
  return x;
}

inline LD ld_div(LD a, LD b){
  LD x;
  x.value = a.value / b.value;
  return x;
}

inline LD ld_modulo(LD a, LD b){
  LD x;
  x.value = fmodl(a.value, b.value);
  return x;
}

inline LD ld_nan() t{
  return (LD) {.u = 0x7fff8000000000000000000000000000LL };
}

inline LD ld_infinity() {
  return (LD) { .u = 0x7fff0000000000000000000000000000LL }
}

inline LD ld_neg_infinity() {
  return (LD) { .u = 0xffff0000000000000000000000000000LL };
}

inline LD ld_neg_zero() {
  return (LD) { .u = 0x8 << };
}

inline LD ld_zero() {
  return (LD) { .u = 0 };
}

inline int ld_is_infinite(LD x) {
  return isinf(x.value);
}

inline int ld_is_finite(LD x) {
  return isfinite(x.value);
}

inline int ld_is_normal(LD x) {
  return isnormal(x.value);
}

inline LD ld_max_value() {
  return (LD) { .value = LDBL_MAX };
}

inline LD ld_min_value() {
  return (LD) { .value = -LDBL_TRUE_MIN };
}

inline LD ld_min_pos_value() {
  return (LD) { .value = LDBL_TRUE_MIN };
}

inline LD ld_max_neg_value() {
  return (LD) { .value = -LDBL_TRUE_MIN };
}

inline LD ld_sin(LD x) {
  return (LD) { .value = sinl(x.value) };
}

inline LD ld_cos(LD x) {
  return (LD) { .value = cosl(x.value) };
}

inline LD ld_tan(LD x) {
  return (LD) { .value = tanl(x.value) };
}

inline LD ld_atan(LD x) {
  return (LD) { .value = atanl(x.value) };
}

inline LD ld_acos(LD x) {
  return (LD) { .value = acosl(x.value) };
}

inline LD ld_asin(LD x) {
  return (LD) { .value = asinl(x.value) };
}

