#include <stddef.h>
#include <stdio.h>

struct find_match
{
    int    found;
    size_t start;
    size_t end;
};

extern void*
re_create(char const*);

extern void
re_destroy(void*);

extern int
re_match(void*, char const*);

extern struct find_match
re_find(void*, char const*);

extern void*
re_captures(void*, char const*);

extern void
re_captures_destroy(void*);

extern struct find_match
re_capture_get(void*, size_t);

extern void*
re_find_iter(void*, char const*);

extern void
re_destroy_find_iter(void*);

extern struct find_match
re_find_iter_next(void*);

extern void*
re_captures_iter(void*, char const*);

extern void
re_destroy_captures_iter(void*);

extern void*
re_captures_iter_next(void*);

void
print_found(struct find_match found, char const* in)
{
    if (!found.found)
    {
        puts("<not found>");
        return;
    }

    printf(
        "start: %llu, end: %llu, string: ",
        (unsigned long long)found.start,
        (unsigned long long)found.end
    );

    for (size_t i = found.start; i != found.end; ++i)
    {
        putchar(in[i]);
    }

    puts("");
}

int
main()
{
    void* re = re_create("^[hH]+");

    printf("%d, %d, %d\n", re_match(re, "hhh"), re_match(re, "jjj"), re_match(re, "H"));

    char const* str1 = "hhhlll";
    char const* str2 = "Hlll";
    char const* str3 = "lll";

    print_found(re_find(re, str1), str1);
    print_found(re_find(re, str2), str2);
    print_found(re_find(re, str3), str3);

    void* cap = re_captures(re, str1);

    if (cap)
    {
        print_found(re_capture_get(cap, 0), str1);
        re_captures_destroy(cap);
    }

    re_destroy(re);
}
