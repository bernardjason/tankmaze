# tank maze

drive a tank through a maze. Left right arrow keys. Up arrow to speed up, left shift to slow down
and space to fire

PC build
```
PKG_CONFIG_ALLOW_CROSS=1 cargo build --target x86_64-pc-windows-gnu
```

Linux build (assuming you are running on linux of course..)
```
cargo build
```

<img src="images/Screenshot from 2020-09-12 18-01-21.png" width="30%"/>
<img src="images/Screenshot from 2020-09-12 18-03-29.png" width="30%"/>
<img src="images/Screenshot from 2020-09-12 18-04-22.png" width="30%"/>

## raspberry pi
If you install all of these plus Rust
* libbz2-dev/stable,now 1.0.6-9.2~deb10u1 armhf [installed]
* libjbig-dev/stable,now 2.1-3.1+b2 armhf [installed]
* libjpeg-dev/stable,now 1:1.5.2-2 all [installed]
* liblzma-dev/stable,now 5.2.4-1 armhf [installed]
* libsdl2-dev/testing,now 2.0.9+dfsg1-1+rpt1 armhf [installed]
* libsdl2-gfx-dev/stable,now 1.0.4+dfsg-3 armhf [installed]
* libsdl2-image-2.0-0/stable,now 2.0.4+dfsg1-1+deb10u1 armhf [installed,automatic]
* libsdl2-image-dev/stable,now 2.0.4+dfsg1-1+deb10u1 armhf [installed]
* libsdl2-ttf-dev/stable,now 2.0.15+dfsg1-1 armhf [installed]
* libtiff-dev/stable,now 4.1.0+git191117-2~deb10u1 armhf [installed]
* libwebp-dev/stable,now 0.6.1-2 armhf [installed]
* libxext-dev/stable,now 2:1.3.3-1+b2 armhf [installed]
* libzstd1/stable,now 1.3.8+dfsg-3+rpi1 armhf [installed]
* libzstd-dev/stable,now 1.3.8+dfsg-3+rpi1 armhf [installed]

you can compile and run on a Raspberry Pi 3 (tested) by changing in Cargo.toml
```
features = ["image","gfx" , "ttf" , "static-link","bundled" ]
```
to
```
features = ["image","gfx" , "ttf" ]
```
which dynamically links. Didn't have inclination to figure out dependencies for static linking from 64bit Ubuntu. 
