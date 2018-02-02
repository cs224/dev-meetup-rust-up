
#include <iostream>
#include <string>

#include "cc_factorial.h"

int main(int argc, char* argv[]) {

    if (argc != 2) {
        std::cerr << "Usage: " << argv[0] << " N" << std::endl;
        std::cerr << "Example: " << argv[0] << " 4" << std::endl;
        return 1;
    }

    std::string snr = argv[1];
    long nr = std::stol(snr, NULL, 10);

    std::cout << factorial(nr) << std::endl;

    return 0;
}