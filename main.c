#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

#define ECIC_DEBUGGING 1

#define ECIC_DEBUG(fmt, ...) \
	do { if (ECIC_DEBUGGING) fprintf(stderr, fmt, __VA_ARGS__); } while (0)

static void 
print_usage_and_exit()
{
	printf("Usage: eci-cleanup infile.wast\n");
	exit(EXIT_FAILURE);
}

static int
program_main(const char *infile)
{
	ECIC_DEBUG("Attempting to open input file: %s\n", infile);
	return 0;
}

int 
main(int argc, char **argv)
{
	if (argc < 2)
		print_usage_and_exit();
	
	char infile[256] = { };
	strncpy(infile, argv[1], strlen(argv[1]));

	program_main(infile);

	return EXIT_SUCCESS;
}
