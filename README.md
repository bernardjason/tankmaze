# tank maze

drive a tank through a maze. Left right arrow keys. Up arrow to speed up, left shift to slow down
and space to fire

PC build
```
PKG_CONFIG_ALLOW_CROSS=1 cargo build --target x86_64-pc-windows-gnu  --features soundoff
```

Linux build (assuming you are running on linux of course..)
```
cargo build
```

