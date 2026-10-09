// Reports the layout of the types in rpmalloc.h back to the Rust `layout` test, so that a
// repeat of the 1.4 -> 2.0 mismatch - where rpmalloc_global_statistics_t silently grew two
// fields and the C code wrote past the Rust struct - fails the test suite instead.

#include "rpmalloc.h"

#include <stddef.h>

size_t
rpmalloc_sys_sizeof(int index) {
	switch (index) {
		case 0:
			return sizeof(rpmalloc_global_statistics_t);
		case 1:
			return sizeof(rpmalloc_thread_statistics_t);
		case 2:
			return sizeof(rpmalloc_interface_t);
		case 3:
			return sizeof(rpmalloc_config_t);
		default:
			return 0;
	}
}

size_t
rpmalloc_sys_offsetof_config_unmap_on_finalize(void) {
	return offsetof(rpmalloc_config_t, unmap_on_finalize);
}

size_t
rpmalloc_sys_offsetof_interface_error_callback(void) {
	return offsetof(rpmalloc_interface_t, error_callback);
}
