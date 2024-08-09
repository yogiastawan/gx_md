

///! This is main commnet
///! used to test
///! llf;;=>>>>>>>>

#ifndef _SYSTEM_
#define _SYSTEM_

//! this is must not included in main commnet
#include <system.h>

#include "header0.h"
//! this is include 1
//!  file in
#include "header1.h"

struct Obj {
  int a;
  int b;
  int c;
};

//! Name alias Obj to Jbo
//! second row
typedef struct Obj Jbo;

//! this comm
//! must not included

typedef struct _Obj_ {
  int e;
  int f;
  int g;
} TestObj;

//! this is test function
//! with many args
int test_function(int a, Obj *obj);

//! this is test function
//! with one args
double test_fun(void);

#endif
