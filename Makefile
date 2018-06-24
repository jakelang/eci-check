CC = clang
CFLAGS = -Wall -Wextra -Werror

OBJ = main.o
BIN = eci-conform

all: eci-conform

eci-conform: $(OBJ)
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f $(OBJ)
	rm -f $(BIN)

