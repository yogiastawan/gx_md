### **Objects:**
* #### **struct Obj**
	```c
	struct Obj{
		a int;
		b int;
		c int;
	};
	```
* #### **Jbo**
	```c
	struct Obj{
		*PRIVATE FIELD*
	};
	typedef struct Obj Jbo;
	```

	Name alias Obj to Jbo second row
* #### **TestObj**
	```c
	struct _Obj_{
		e int;
		f int;
		g int;
	};
	typedef struct _Obj_ TestObj;
	```

### **Functions:**
* #### **test_function**
	```c
	int test_function(int a, Obj *obj);
	```

	this is test function with many args
* #### **test_fun**
	```c
	double test_fun(void);
	```

	this is test function with one args

### **Includes:**
* [header0.h](#)
* [header1.h](#)\
	this is include 1 file in
#### **Objects (3)**
* [struct Obj](#struct-obj)
* [Jbo](#jbo)
* [TestObj](#testobj)
#### **Functions (2)**
* [test_function](#test_function)
* [test_fun](#test_fun)
#### **Includes (2)**
* [header0.h](#)
* [header1.h](#)
