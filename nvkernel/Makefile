AS = nasm
CC = nvs                     # Navescript compiler (to x86_64)
LD = ld
CFLAGS = -target x86_64-elf
LDFLAGS = -T boot/linker.ld -ffreestanding -nostdlib
ASFLAGS = -f elf64

OBJS = boot/multiboot2.o boot/x86_64.o \
       src/main.o src/print.o src/gdt.o src/idt.o src/interrupt.o \
       src/memory.o src/scheduler.o src/syscall.o src/keyboard.o

all: nvkernel.elf

%.o: %.asm
	$(AS) $(ASFLAGS) $< -o $@

%.o: %.ns
	$(CC) $(CFLAGS) -c $< -o $@

nvkernel.elf: $(OBJS)
	$(LD) $(LDFLAGS) -o $@ $(OBJS)

run: nvkernel.elf
	qemu-system-x86_64 -kernel nvkernel.elf -serial mon:stdio

clean:
	rm -f *.o src/*.o boot/*.o nvkernel.elf
