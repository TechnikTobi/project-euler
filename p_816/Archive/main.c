#include <stdio.h>
#include "wrapper.h"

int main
()
{
	int result = rust_function(2, 4);
	printf("Hello! %d\n", result);
	return 0;
}
