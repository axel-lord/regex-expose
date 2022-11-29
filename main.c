#include <stdio.h>

extern void*
re_create(char const*);

extern void
re_destroy(void*);

int
main()
{
    void* re = re_create("^h*");
    puts("Hello world!");
    re_destroy(re);
}
