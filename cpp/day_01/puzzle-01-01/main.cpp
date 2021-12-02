//
// Created by me on 01/12/2021.
//

#include <string>
#include <iostream>

int main(int argc, char *argv[]) {
    int previous = 0;
    int accumulator = -1;

    for (std::string line; std::getline(std::cin, line);) {
        int value = std::stoi(line);

        if (value > previous) {
            accumulator++;
        }

        previous = value;
    }

    std::cout << "Answer: " << accumulator << std::endl;

    return 0;
}