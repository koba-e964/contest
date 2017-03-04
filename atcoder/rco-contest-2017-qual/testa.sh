make CXXFLAGS=-O2 a || exit
cd a-tester
java Generator -seed 11425 >input.txt
time ../a <input.txt >output.txt
java Judge input.txt output.txt
