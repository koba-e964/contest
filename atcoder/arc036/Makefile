CLASSPATH = "./;/p32/scala/lib/scala-library.jar" # In windows, separators are semicolons.
%.exe:%.cpp
	g++ $< -o $@
%.jx:%.scala
	fsc $<
scx: Main.class # Execute Main.class compiled by Scala
	java -classpath $(CLASSPATH) Main