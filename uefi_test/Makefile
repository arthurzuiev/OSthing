TARGET = build/hello.efi
SRC = src/main.c
CFLAGS = -O2 -Wall -ffreestanding -fshort-wchar -IC:/UEFI-Headers/MdePkg/Include
LDFLAGS = -nostdlib -Wl,-subsystem,10

all: $(TARGET)

$(TARGET): $(SRC)
    x86_64-w64-mingw32-gcc $(CFLAGS) $(SRC) -o $(TARGET) $(LDFLAGS)

clean:
    rmdir /s /q build
