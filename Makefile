CC = clang
CFLAGS = -Wall -Wextra -Werror

OBJ = main.o
BIN = eci-cleanup

all: eci-cleanup

eci-cleanup: $(OBJ)
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f $(OBJ)
	rm -f $(BIN)

