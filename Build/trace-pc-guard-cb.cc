// trace-pc-guard-cb.cc
#include <sys/types.h>
#include <sys/shm.h>
#include <string.h>
#include <stdint.h>
#include <stdio.h>
#include <sanitizer/coverage_interface.h>
static uint64_t N;

struct shmstr {
    uint32_t count;
    uint32_t buf[1024];

};
int shmid = shmget(7015,4100,0644|IPC_CREAT);
struct shmstr *shmaddr = (struct shmstr*)shmat(shmid,NULL,0);
// This callback is inserted by the compiler as a module constructor
// into every DSO. 'start' and 'stop' correspond to the
// beginning and end of the section with the guards for the entire
// binary (executable or DSO). The callback will be called at least
// once per DSO and may be called multiple times with the same parameters.
extern "C" void __sanitizer_cov_trace_pc_guard_init(uint32_t *start,
                                                            uint32_t *stop) {
        if (start == stop || *start) return;  // Initialize only once.
          //printf("INIT: %p %p\n", start, stop);
            for (uint32_t *x = start; x < stop; x++)
                *x = N+1;  // Guards should start from 1.

}

// This callback is inserted by the compiler on every edge in the
// control flow (some optimizations apply).
// Typically, the compiler will emit the code like this:
//    if(*guard)
//      __sanitizer_cov_trace_pc_guard(guard);
// But for large functions it will emit a simple call:
//    __sanitizer_cov_trace_pc_guard(guard);
extern "C" void __sanitizer_cov_trace_pc_guard(uint32_t *guard) {
    if (!*guard) {
    //printf("Shmid: %d \nRepeated :%x\n",shmid,(uint64_t)guard&0x0000ff);
    //shmaddr->buf[65536-((uint64_t)guard&0xffff)]++;
    shmaddr->buf[(uint64_t)guard&0x0000ff]++;
    shmaddr->count=N++;
    printf("\nCount %lu\n",N);
    // //printf("%p",shmaddr);
    // The values of `*guard` are as you set them in
    // __sanitizer_cov_trace_pc_guard_init and so you can make them consecutive
    } 

    else{
    // and use them to dereference an array or a bit vector.
    void *PC = __builtin_return_address(0);
    char PcDescr[1024];
    // This function is a part of the sanitizer run-time.
    // To use it, link with AddressSanitizer or other sanitizer.
    //printf("\naddress %d\n",(uint64_t)guard&0x0000ff);
    //printf("\naddress value %d\n",shmaddr->buf[(uint64_t)guard&0x0000ff]);
    shmaddr->buf[(uint64_t)guard&0x0000ff]++;
    shmaddr->count=N++;
        
    __sanitizer_symbolize_pc(PC, "%p %F %L", PcDescr, sizeof(PcDescr));
    printf("\nCount %lu\n",N);
    //printf("Guard: %p %x PC %s\n",guard , *guard, PcDescr);
}
}
