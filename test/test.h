#ifndef _SYSTEM_
#define _SYSTEM_

#include <system.h>"

#include "header0.h"
#include "header1.h"

struct Obj {
  int a;
  int b;
  int c;
};

typedef struct Obj Jbo;

typedef struct _Obj_ {
  int e;
  int f;
  int g;
} TestObj;

int test_function(int a, Obj *obj);

double test_fun(void);

#endif
