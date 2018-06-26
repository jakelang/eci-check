CC = clang
AR = llvm-ar
LD = lld

CFLAGS = -Wall -Wextra -Werror

BIN = eci-conform

CLI-OBJ = $(OBJ) eci-conform.o
OBJ = ecic.o

all: cli archive

cli: eci-conform
archive: libecic.a

eci-conform: $(CLI-OBJ)
	$(CC) $(CFLAGS) -o $@ $^

libecic.a: $(OBJ)
	$(AR) -r $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f *.o
	rm -f *.a
	rm -f $(BIN)
