# Arbitarily Read and Write Files in Rust

If you have ever used C/C++, you must have certain recognization of their file I/O. In which, we open a file and get a file descriptor which is also a file pointer. With the file pointer, we can arbitraily read any number of bytes we want from the file. While reading and writing, the file pointer moves certain bytes too. And you can use the `seek` function to manually move the file pointer. 

In Rust, we as free as in C/C++. When we open a file in Rust, if successfully, we get a `std::fs::File` reference. The `File` type implements three traits for file operations: `Read`, `Write` and `Seek`. The three traits provide many methods, but mostly used methods of them are `read`, `write`, and `seek` respectively.

If we want to read, we need to provide a buffer, and call the `read` method on the `File` reference will fill the buffer with bytes read from the file if the length of the file is longer than the buffer. If not, read all bytes of the file and fill them in the buffer. No matter which scenarios, the `read` will **finally return the number of bytes it reads.**

The `write` method is almost the same, it receives a reference of a buffer, and write the buffer into a file. You can pass a slice of an array or vector in.  Number of bytes written will be returned.

The `seek` method helps us manually move the file pointer. We have to specify where to move the file pointer relative to. Three locations are provided in the `SeekFrom` enum: `Start`, `End`, and `Current`. These enums are all able to store a `i64` value, which is the number of bytes we want to move.

