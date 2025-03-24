#! /bin/bash
cd ./resources/shaders/
if [ -d bin ]; then
  echo "Found shaders/bin directory"
else 
  mkdir bin;
fi

for file in *.glsl ; do
  name="${file%.*}"
  glslc "$file" -o "./bin/${name}.spv"
done
