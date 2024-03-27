# Rust implementation of an asynchronous pool of resources.

The following steps had to be taken to make it compile on Windows
1. use the following toolchain: stable-x86_64-pc-windows-msvc
2. install OpenSSL through winget
3. set the following two environment variables:

	$env:OPENSSL_LIB_DIR => C:\Program Files\OpenSSL-Win64\lib\VC\x64\MD\
	$env:OPENSSL_DIR => C:\Program Files\OpenSSL-Win64\

There are no issues on Linux

