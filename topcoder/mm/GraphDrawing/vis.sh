#!/bin/sh
make CXXFLAGS="-Wall -O2 -DDEBUG" GraphDrawing
java -jar Tester.jar -exec "./GraphDrawing" -seed 2 -vis
