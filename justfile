default:
    just --list

alias g := generate-c
alias run := run-bin

# Runs the generated binary from compiling the generated C code
run-bin input-file: (compile-c input-file)
    ./generated/`basename -s .syml {{input-file}}`-binary

compile-c input-file: (generate-c input-file)
    cc generated/`basename -s .syml {{input-file}}`.c -Wall -std=c11 -o generated/`basename -s .syml {{input-file}}`-binary

# If you want multiple files you could space-separate them as multiple args to cat
generate-c input-file:
    cat {{input-file}} | cargo run > generated/`basename -s .syml {{input-file}}`.c

print-c input-file:
    cat {{input-file}} | cargo run

clean:
    rm -f ./generated/*

#----------------------------------------
# Some silly one-liners if you prefer:
# cat samplecode/one.syml | cargo run > output.c; gcc output.c -o c-binary ; c-binary
# cat samplecode/one.syml | cargo run | gcc -x c -o c-binary - ; c-binary



# # Define a variable with your full path
# FILE_PATH := "/home/user/project/data.txt"

# # A recipe to print just the filename
# filename:
#   # Use shell command 'basename  -s .syml' in a backtick
#   @echo "The filename is: `basename  -s .syml {{FILE_PATH}}`"

# # A recipe to get the filename without the extension
# filename-no-ext:
#   # Use shell parameter expansion to remove extension
#   @echo "The filename without extension is: `basename  -s .syml -s .txt {{FILE_PATH}}`"
