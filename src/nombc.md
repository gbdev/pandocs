# No MBC

(32 KiB ROM only)

Small games of not more than 32 KiB ROM do not require a MBC chip for
ROM banking. The ROM is directly mapped to memory at $0000-7FFF.
Optionally up to 8 KiB of RAM could be connected at $A000-BFFF, using
a discrete logic decoder <!--74HC138?--> in place of a full MBC chip.
