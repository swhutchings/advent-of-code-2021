
#include <deque>
#include <string>
#include <iostream>
#include <numeric>

using namespace std;

const unsigned int WINDOW_WIDTH = 3;

int main(int argc, char *argv[]) {
    deque<int> buffer;
    int previous = 0;
    int accumulator = -1;

    for (string line; getline(cin, line);) {
        int value = stoi(line);

        buffer.push_front(value);

        if (buffer.size() > WINDOW_WIDTH) {
            buffer.pop_back();
        }

        if (buffer.size() == WINDOW_WIDTH) {
            int sum = accumulate(buffer.begin(), buffer.end(), 0);

            if (sum > previous) {
                accumulator++;
            }

            previous = sum;
        }
    }

    std::cout << "Answer: " << accumulator << std::endl;

    return 0;
}