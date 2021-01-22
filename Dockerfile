# Use the container that comes with `cross` as a base. It's already got
# a cross-compile toolchain installed, so that's less work for us.
FROM rustembedded/cross:arm-unknown-linux-gnueabihf-0.2.1

RUN apt-get update
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install --assume-yes wget

RUN wget "http://192.168.0.10:8000/libraspberrypi-dev_1.20210108-1_armhf.deb"
RUN wget "http://192.168.0.10:8000/libraspberrypi0_1.20210108-1_armhf.deb"
RUN wget "http://192.168.0.10:8000/raspberrypi-bootloader_1.20210108-1_armhf.deb"

RUN dpkg -i raspberrypi-bootloader_1.20210108-1_armhf.deb
RUN dpkg -i libraspberrypi0_1.20210108-1_armhf.deb
RUN dpkg -i libraspberrypi-dev_1.20210108-1_armhf.deb