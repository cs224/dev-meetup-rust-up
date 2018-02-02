//
// Created by cs on 30.01.18.
//

#include "cc_factorial.h"

long factorial(long n) {
    long result = 1;
    for(long i=1;i<=n;i++) {
        result *= i;
    }
    return result;
}

