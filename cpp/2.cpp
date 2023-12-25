#include <iostream>
#include <fstream>
#include <string>
#include <map>
#include <string_view>

void part1();
void part2();

int main() {
    part1();
    part2();
    return 0;
}

void part1() {
    std::ifstream ifs("inputs/2.txt");
    std::string line;
    std::string::size_type game_num = 1;
    std::string::size_type sum = 0;
    while (std::getline(ifs, line)) {
        bool valid = true;

        int draw_start_index = line.find(':') + 1;
        int draw_end_index;
        while (valid && draw_start_index < line.size()) {
            draw_end_index = line.find(';', draw_start_index);

            if (draw_end_index == std::string::npos) {
                draw_end_index = line.size();
            }

            std::string_view sv = std::string_view(line.data() + draw_start_index, draw_end_index - draw_start_index);

            int num_red = 0;
            int num_green = 0;
            int num_blue = 0;

            int balls_start_index = 0;
            int balls_end_index;
            while (balls_start_index < sv.size()) {
                balls_end_index = sv.find(',', balls_start_index);

                if (balls_end_index == std::string::npos) {
                    balls_end_index = sv.size();
                }

                std::string_view ball = std::string_view(sv.data() + balls_start_index, balls_end_index - balls_start_index);
                // Format is " -<number> <color>"
                int space_index = ball.find(' ', 1);
                int num = std::stoi(std::string(ball.data(), space_index)); // todo: dont copy

                if (ball.find("red") != std::string::npos) {
                    num_red += num;
                } else if (ball.find("green") != std::string::npos) {
                    num_green += num;
                } else if (ball.find("blue") != std::string::npos) {
                    num_blue += num;
                } else {
                    std::cout << "Unknown color: " << ball << std::endl;
                }



                balls_start_index = balls_end_index + 1;
            }

            if (num_red > 12 || num_green > 13 || num_blue > 14) {
                valid = false;
                break;
            }

            draw_start_index = draw_end_index + 1;
        }

        if (valid) {
            sum += game_num;
        }

        game_num++;
    }
    std::cout << "Part 1 Answer: " << sum << std::endl;
}

void part2() {
    std::ifstream ifs("2.txt");
    std::string line;
    std::string::size_type game_num = 1;
    std::string::size_type sum = 0;
    while (std::getline(ifs, line)) {
        int min_red = 0;
        int min_green = 0;
        int min_blue = 0;

        int draw_start_index = line.find(':') + 1;
        int draw_end_index;
        while (draw_start_index < line.size()) {
            draw_end_index = line.find(';', draw_start_index);

            if (draw_end_index == std::string::npos) {
                draw_end_index = line.size();
            }

            std::string_view sv = std::string_view(line.data() + draw_start_index, draw_end_index - draw_start_index);
            
            int balls_start_index = 0;
            int balls_end_index;

            int num_red = 0;
            int num_green = 0;
            int num_blue = 0;
            while (balls_start_index < sv.size()) {
                balls_end_index = sv.find(',', balls_start_index);

                if (balls_end_index == std::string::npos) {
                    balls_end_index = sv.size();
                }

                std::string_view ball = std::string_view(sv.data() + balls_start_index, balls_end_index - balls_start_index);
                // Format is " <number> <color>"
                int space_index = ball.find(' ', 1);
                int num = std::stoi(std::string(ball.data(), space_index)); // todo: dont copy

                if (ball.find("red") != std::string::npos) {
                    num_red += num;
                } else if (ball.find("green") != std::string::npos) {
                    num_green += num;
                } else if (ball.find("blue") != std::string::npos) {
                    num_blue += num;
                } else {
                    std::cerr << "Unknown color: " << ball << std::endl;
                    exit(0);
                }



                balls_start_index = balls_end_index + 1;
            }

            min_red = std::max(min_red, num_red);
            min_green = std::max(min_green, num_green);
            min_blue = std::max(min_blue, num_blue);

            draw_start_index = draw_end_index + 1;
        }

        sum += (min_red * min_green * min_blue);

        game_num++;
    }
    std::cout << "Part 2 Answer: " << sum << std::endl;
}