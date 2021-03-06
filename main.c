#include <stdio.h>
#include "counter.h"

int fake_main(int argc, char* argv[]) {
    /* make sure not to call rust code from other threads though, the
       rust runtime was only initialized for this one... */
    Counter* c = counter_new(5);
    int i;

    while ((i = counter_wait(c)) != -1) {
      printf("%d\n", i);
    }

    counter_free(c);

    return 0;
}

int main(int argc, char* argv[]) {
    return run_with_runtime(argc, argv, fake_main);
}
