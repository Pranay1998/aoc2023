find . -name "*.cpp" -exec sh -c 'g++ -o "out/$(basename {} .cpp)" "{}" && echo "Compiled {} successfully" || echo "Error compiling {}"' \;
