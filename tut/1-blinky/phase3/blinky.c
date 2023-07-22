#define GPIO_BASE (0x3F000000 + 0x200000)

volatile unsigned *GPIO_FSEL1 = (volatile unsigned *)(GPIO_BASE + 0x04);
volatile unsigned *GPIO_SET0  = (volatile unsigned *)(GPIO_BASE + 0x1C);
volatile unsigned *GPIO_CLR0  = (volatile unsigned *)(GPIO_BASE + 0x28);

static void spin_sleep_us(unsigned int us) {
  for (unsigned int i = 0; i < us * 6; i++) {
    asm volatile("nop");
  }
}

static void spin_sleep_ms(unsigned int ms) {
  spin_sleep_us(ms * 1000);
}

int kmain(void) {
  // FIXME: STEP 1: Set GPIO Pin 16 as output.
  // GPIO FSEL1 bits 20-18 need to be 001
  // clear bits first
  *GPIO_FSEL1 = (*GPIO_FSEL1 & ~(0b111 << 18));
  // set the bits to 001
  *GPIO_FSEL1 = (*GPIO_FSEL1 | (0b1 << 18));

  // FIXME: STEP 2: Continuously set and clear GPIO 16.
  while (1) {
    *GPIO_SET0 = (0b1 << 16);
    spin_sleep_ms(250);
    *GPIO_CLR0 = (0b1 << 16);
    spin_sleep_ms(250);
  }
}
