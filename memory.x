MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* STM32L476RG: 1MB Flash, 96KB SRAM1 + 32KB SRAM2 */
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM   : ORIGIN = 0x20000000, LENGTH = 96K
  RAM2  : ORIGIN = 0x10000000, LENGTH = 32K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full size of the RAM, all the way up to the top of the RAM. */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
