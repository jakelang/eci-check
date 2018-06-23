#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

static void 
print_usage_and_exit()
{
	printf("Usage: eci-cleanup infile.wast\n");
	exit(EXIT_FAILURE);
}

int 
main(int argc, char **argv)
{
	if (argc < 2)
		print_usage_and_exit();
}
