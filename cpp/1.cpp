// read 1.txt line by line
// and print the line number and the line content

#include <iostream>
#include <fstream>
#include <string>
#include <map>

void part1();
void part2();

int main() {
    part1();
    part2();
    return 0;
}

void part1() {
    std::ifstream ifs("inputs/1.txt");
    std::string line;
    int sum = 0;
    while (std::getline(ifs, line)) {
        char *firstDigit = nullptr;
        char *lastDigit = nullptr;
        for (int i = 0; i < line.size(); i++) {
            if (line[i] >= '0' && line[i] <= '9') {
                if (firstDigit == nullptr) {
                    firstDigit = &line[i];
                }
                lastDigit = &line[i];
            }
        }
        if (firstDigit != nullptr && lastDigit != nullptr) {
            sum += (*firstDigit - '0') * 10 + (*lastDigit - '0');
        }
    }
    std::cout << "Part 1 Answer: " << sum << std::endl;
}

void part2() {
    // create a map of string to string
    std::map<std::string, int> numbersMapping;

    numbersMapping["zero"] = 0;
    numbersMapping["one"] = 1;
    numbersMapping["two"] = 2;
    numbersMapping["three"] = 3;
    numbersMapping["four"] = 4;
    numbersMapping["five"] = 5;
    numbersMapping["six"] = 6;
    numbersMapping["seven"] = 7;
    numbersMapping["eight"] = 8;
    numbersMapping["nine"] = 9;

    std::ifstream ifs("1.txt");
    std::string line;
    int sum = 0;
    while (std::getline(ifs, line)) {

        int firstDigit;
        int firstDigitIndex = -1;
        int lastDigit;
        int lastDigitIndex = -1;
        for (int i = 0; i < line.size(); i++) {
            if (line[i] >= '0' && line[i] <= '9') {
                if (firstDigitIndex == -1) {
                    firstDigit = line[i] - '0';
                    firstDigitIndex = i;
                }
                lastDigit = line[i] - '0';
                lastDigitIndex = i;
            }
        }

        for (const auto &pair: numbersMapping) {
            std::string::size_type n = 0;
            while (std::string::npos != (n = line.find(pair.first, n))) {
                if (n < firstDigitIndex || firstDigitIndex == -1) {
                    firstDigitIndex = n;
                    firstDigit = pair.second;
                }
                if (n > lastDigitIndex || lastDigitIndex == -1) {
                    lastDigitIndex = n;
                    lastDigit = pair.second;
                }
                n = n + pair.first.size();
            } 
        }

        sum += (firstDigit * 10) + lastDigit;
    }
    std::cout << "Part 2 Answer: " << sum << std::endl;
}