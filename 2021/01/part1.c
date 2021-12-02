#include <stdio.h>
#include "measurements.h"

int main()
{
    int measurements[2000] = {DATA};

    int increased = 0;

    for (int i = 1; i < 2000; i++) {
        if (measurements[i - 1] < measurements[i]) {
            increased++;
        }
    }

    printf("Answer: %d\n", increased);
    return 0;
}
