#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct AbiMethods_Container_i32;
struct AbiClass_Container_i32;
struct AbiMethods_Option_i32;
struct AbiClass_Option_i32;
struct Hicc_no_std_demo;

typedef struct AbiMethods_Container_i32 {
  void (*hicc_destroy)(struct AbiClass_Container_i32);
  struct AbiClass_Container_i32 (*hicc_make_unique)(struct AbiClass_Container_i32);
  struct AbiClass_Container_i32 (*hicc_make_ref_mut)(struct AbiClass_Container_i32*);
  uintptr_t (*hicc_size_of)(void);
  void (*hicc_write)(struct AbiClass_Container_i32*, struct AbiClass_Container_i32);
  struct AbiClass_Container_i32 (*hicc_make_ref)(const struct AbiClass_Container_i32*);
  const int32_t *(*get)(struct AbiClass_Container_i32*);
} AbiMethods_Container_i32;

typedef struct AbiClass_Container_i32 {
  const struct AbiMethods_Container_i32 *methods;
  const void *this_;
  uintptr_t level;
} AbiClass_Container_i32;

typedef struct AbiMethods_Option_i32 {
  void (*hicc_destroy)(struct AbiClass_Option_i32);
  struct AbiClass_Option_i32 (*hicc_make_unique)(struct AbiClass_Option_i32);
  struct AbiClass_Option_i32 (*hicc_make_ref_mut)(struct AbiClass_Option_i32*);
  uintptr_t (*hicc_size_of)(void);
  void (*hicc_write)(struct AbiClass_Option_i32*, struct AbiClass_Option_i32);
  struct AbiClass_Option_i32 (*hicc_make_ref)(const struct AbiClass_Option_i32*);
  bool (*is_none)(struct AbiClass_Option_i32*);
  int32_t (*unwrap)(struct AbiClass_Option_i32);
  struct AbiClass_Option_i32 (*take)(struct AbiClass_Option_i32*);
  const int32_t *(*as_ref)(struct AbiClass_Option_i32*);
  int32_t *(*as_mut)(struct AbiClass_Option_i32*);
} AbiMethods_Option_i32;

typedef struct AbiClass_Option_i32 {
  const struct AbiMethods_Option_i32 *methods;
  const void *this_;
  uintptr_t level;
} AbiClass_Option_i32;

typedef struct Hicc_no_std_demo {
  int32_t (*add)(int32_t, int32_t);
  int32_t (*negate)(int32_t);
  int32_t (*container_value)(struct AbiClass_Container_i32);
  struct AbiClass_Container_i32 (*new_container)(int32_t);
  int64_t (*double_option)(struct AbiClass_Option_i32);
  struct AbiClass_Option_i32 (*new_option)(int32_t);
} Hicc_no_std_demo;

const struct Hicc_no_std_demo *no_std_demo(void);
