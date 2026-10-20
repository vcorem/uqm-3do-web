/* Compare the actual translator output with independently compiled source. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
extern uint32_t macro_fixture(uint32_t, uint32_t, uint32_t);
extern uint32_t oracle_macro_fixture(uint32_t, uint32_t, uint32_t);
void rust_eh_personality(void) { abort(); }
static uint32_t random_state = UINT32_C(0x31415926);
static uint32_t next(void) {
    random_state ^= random_state << 13;
    random_state ^= random_state >> 17;
    random_state ^= random_state << 5;
    return random_state;
}
int main(void) {
    uint64_t comparisons = 0, mismatches = 0;
    const uint32_t edges[] = {0,1,2,255,256,329,32767,32768,65535,65536,0x7fffffff,0x80000000,0xffffffff};
    for (uint32_t selector = 0; selector < 16; selector++) {
        for (unsigned i=0;i<sizeof edges/sizeof edges[0];i++) for (unsigned j=0;j<sizeof edges/sizeof edges[0];j++) {
            uint32_t a=edges[i],b=edges[j],expected=oracle_macro_fixture(selector,a,b),actual=macro_fixture(selector,a,b);
            comparisons++;
            if(actual!=expected) {
                if(!mismatches)fprintf(stderr,"Macro mismatch selector=%u a=%u b=%u C=%u Rust=%u\n",selector,a,b,expected,actual);
                mismatches++;
            }
        }
        for(unsigned i=0;i<10000;i++) {
            uint32_t a=next(),b=next();comparisons++;
            if(macro_fixture(selector,a,b)!=oracle_macro_fixture(selector,a,b))mismatches++;
        }
    }
    printf("Macro oracle: %llu comparisons, %llu mismatches\n",(unsigned long long)comparisons,(unsigned long long)mismatches);
    return mismatches ? 1 : 0;
}
