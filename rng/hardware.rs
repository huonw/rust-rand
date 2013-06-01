// Implement support for the RDRAND instruction on x86-64. Detect that
// it is supported via `cpuinfo(eax = 1)` and checking `%ecx & (1 <
// 30)`