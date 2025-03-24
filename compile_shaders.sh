#! /bin/bash
cd ./resources/shaders/
for file in *.glsl ; do
  name="${file%.*}"
  glslc "$file" -o "./bin/${name}.spv"
done
