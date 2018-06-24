#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

#include "ecic.h"

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
	
	char infile[256] = { };
	strncpy(infile, argv[1], strlen(argv[1]));

	ecic_main(infile);

	return EXIT_SUCCESS;
}
