make CXXFLAGS=-O2 b || exit
cd b-tester
java Generator -seed 11414 >input.txt
time ../b <input.txt >output.txt
java Judge input.txt output.txt
