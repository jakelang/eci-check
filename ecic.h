#ifndef __ECIC_H
#define __ECIC_H

#define ECIC_DEBUGGING 1

#include <stdio.h>

#define ECIC_DEBUG(fmt, ...) \
	do { if (ECIC_DEBUGGING) fprintf(stderr, fmt, __VA_ARGS__); } while (0)

int ecic_main(const char *infile);

#endif
