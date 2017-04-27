#!/bin/sh
make CXXFLAGS="-Wall -O2 -DDEBUG" GraphDrawing
javac GraphDrawingVis.java
java GraphDrawingVis -exec "./GraphDrawing"
