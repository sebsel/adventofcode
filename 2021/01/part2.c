#include <stdio.h>
#include "measurements.h"

int main()
{
    int meas[2000] = {DATA};

    int increased = 0;
    int prevSum, currSum;

    for (int i = 3; i < 2000; i++) {
        prevSum = meas[i - 3] + meas[i - 2] + meas[i - 1];
        currSum = meas[i - 2] + meas[i - 1] + meas[i];

        if (currSum > prevSum) {
            increased++;
        }
    }

    printf("Answer: %d\n", increased);
    return 1;
}
