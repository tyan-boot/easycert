# easycert
Simple zero-config local certificates generator in rust

```shell
$ easycert init -l 4096
new ca generated!

$ easycert new example.com *.example.com 10.0.0.1 ::1
New cert created to example.com.key example.com.crt

$ openssl x509 -in example.com.crt -text -noout
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number:
            af:38:f9:54
    Signature Algorithm: sha512WithRSAEncryption
        Issuer: CN = TYANBOOT
        Validity
            Not Before: Jul 22 10:06:51 2018 GMT
            Not After : Jul 19 10:06:51 2028 GMT
        Subject: CN = example.com
        Subject Public Key Info:
            Public Key Algorithm: rsaEncryption
                Public-Key: (4096 bit)
                Modulus:
                    00:dc:0e:35:c8:bb:ec:72:82:eb:f1:df:72:96:f5:
                    ...
                    cb:b8:cf
                Exponent: 65537 (0x10001)
        X509v3 extensions:
            X509v3 Basic Constraints:
                CA:FALSE
            X509v3 Key Usage:
                Digital Signature, Key Encipherment
            X509v3 Authority Key Identifier:
                keyid:75:94:3F:58:9C:33:11:45:4A:0F:7A:CD:FB:FF:6C:9F:2F:AB:28:5A

            X509v3 Subject Key Identifier:
                FD:27:A7:1A:25:57:81:8C:B8:36:6E:C0:96:CF:7F:F0:20:87:B4:B3
            X509v3 Subject Alternative Name:
                DNS:example.com, DNS:*.example.com, IP Address:10.0.0.1, IP Address:0:0:0:0:0:0:0:1
    Signature Algorithm: sha512WithRSAEncryption
         3d:2a:7a:2e:80:b4:03:37:03:45:00:91:47:cf:42:c9:4a:71:
         ....
         0b:f2:ca:00:7a:b6:86:a5
```
<p align="center"><img alt="img" src="https://i.loli.net/2018/07/22/5b54583ce5901.png"></p>

# Installation

## Download pre-built binary from Github release

## Build from source

### Pre-requirements
* rust environment
* openssl develop package (lib etc)

### Linux

__build__
```shell
$ cargo build --release
$ target/release/easycert
```

### Windows

#### MSVC (recommend)
You need install msvc first, either full visual studio or msvc only. This is required by rust. For details please refer to rust installation.


__Install openssl__

__Option 1__

[precompiled binaries](http://slproweb.com/products/Win32OpenSSL.html)

__Option 2__

[vcpkg](https://github.com/Microsoft/vcpkg)

```
vcpkg install openssl:x64-windows
```
if you want to build statically linked easycert, please install `openssl:x64-windows-static` instead

__Option 3 (Not Recommend)__

Build openssl from source can be complex and may waste several minutes in your life :)

If you still want to do that, please refer to:

* [INSTALL](https://github.com/openssl/openssl/blob/master/INSTALL)
* [NOTE](https://github.com/openssl/openssl/blob/master/NOTES.WIN)

__Build__
If you install openssl via vcpkg, just
```
set VCPKG_ROOT=<c:\path\to\vcpkg\installation>
cargo build
```

else if you should set OPENSSL_DIR manually
```
set OPENSSL_DIR=<c:\path\to\openssl>
cargo build
```

If you want to build statically linked executeable, set OPENSSL_STATIC=1 before build.

VC runtime are already statically linked, see `.cargo/config`

# FAQ

## Why my certificates are not trusted by browser?
> You need install ca.pem and ca.key into your system trusted root certificates first, 
the next version will try to install it automatically.

## How to do that?
> Different system has different ways to install, please search for Google and etc for details instruction. 
In Windows, it is required to export to pkcs12 format first.
