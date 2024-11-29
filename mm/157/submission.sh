PROBLEM=RollingBalls

rm *.zip
oj-bundle src/main.cpp >$PROBLEM.cpp
zip $PROBLEM.cpp.zip $PROBLEM.cpp
rm $PROBLEM.cpp
