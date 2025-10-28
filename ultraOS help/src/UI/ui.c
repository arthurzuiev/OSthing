#define VGA_MEMORY 0xB8000
#define VGA_WIDTH 80
#define VGA_HEIGHT 25

void print_string(const char* str, int x, int y, unsigned char color);
void clear_screen();
void draw_taskbar();
void draw_desktop();
void halt();
void main();

void clear_screen() {
    unsigned short* vga = (unsigned short*)VGA_MEMORY;
    for (int i = 0; i < VGA_WIDTH * VGA_HEIGHT; i++)
        vga[i] = (0x1F << 8) | ' '; // Blue background, white text
}

void print_string(const char* str, int x, int y, unsigned char color) {
    unsigned short* vga = (unsigned short*)VGA_MEMORY;
    int offset = y * VGA_WIDTH + x;
    for (int i = 0; str[i]; i++)
        vga[offset + i] = (color << 8) | str[i];
}

void draw_taskbar() {
    unsigned short* vga = (unsigned short*)VGA_MEMORY;
    int y = VGA_HEIGHT - 1;
    for (int x = 0; x < VGA_WIDTH; x++)
        vga[y * VGA_WIDTH + x] = (0x1F << 8) | ' ';

    print_string("[ UltraOS Start ]", 1, y, 0x1E);
    print_string("|  Terminal  |  Files  |  Settings  |", 22, y, 0x1F);
    print_string("12:00", 70, y, 0x1E);
}

void draw_desktop() {
    print_string("UltraOS 64-bit Desktop", 28, 2, 0x1F);
    print_string("───────────────────────────────────────", 22, 3, 0x1F);

    print_string("[📁 My Files]", 5, 6, 0x1F);
    print_string("[💻 Terminal]", 5, 8, 0x1F);
    print_string("[⚙️ Settings]", 5, 10, 0x1F);

    print_string("System Info:", 55, 6, 0x1E);
    print_string("CPU: x86_64", 55, 7, 0x1F);
    print_string("Mode: Long (64-bit)", 55, 8, 0x1F);
    print_string("VGA: 80x25 Text", 55, 9, 0x1F);
}

void main() {
    clear_screen();
    draw_desktop();
    draw_taskbar();
    halt();
}

void halt() {
    while (1) __asm__("hlt");
}
