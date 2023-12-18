mkdir -p foo
find . -name "*.cpp" -exec sh -c 'g++ -std=c++17 -o "out/$(basename {} .cpp)" "{}" && echo "Compiled {} successfully" || echo "Error compiling {}"' \;