#ifndef BLAHBLAH_COUNTER_H
#define BLAHBLAH_COUNTER_H

typedef struct Counter Counter;

Counter* counter_new(int count);
int counter_wait(Counter* counter);
void counter_free(Counter* counter);

#endif /* BLAHBLAH_COUNTER_H */
