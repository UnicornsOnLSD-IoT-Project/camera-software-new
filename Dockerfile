# Use the container that comes with `cross` as a base. It's already got
# a cross-compile toolchain installed, so that's less work for us.
FROM rustembedded/cross:arm-unknown-linux-gnueabihf-0.2.1

ENV TZ=Europe/London
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get upgrade --assume-yes && \
    apt-get install --assume-yes wget libssl-dev:armhf

RUN wget "http://192.168.0.10:8000/libraspberrypi-dev_1.20210108-1_armhf.deb"
RUN wget "http://192.168.0.10:8000/libraspberrypi0_1.20210108-1_armhf.deb"
RUN wget "http://192.168.0.10:8000/raspberrypi-bootloader_1.20210108-1_armhf.deb"

RUN dpkg -i raspberrypi-bootloader_1.20210108-1_armhf.deb
RUN dpkg -i libraspberrypi0_1.20210108-1_armhf.deb
RUN dpkg -i libraspberrypi-dev_1.20210108-1_armhf.deb 
RUN dpkg -i  *.deb

ENV PKG_CONFIG_LIBDIR_arm-unknown-linux-gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV PKG_CONFIG_LIBDIR=/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV PKG_CONFIG_PATH_arm-unknown-linux-gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig
ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
# ENV OPENSSL_LIB_DIR=/usr/lib/arm-linux-gnueabihf
# ENV OPENSSL_INCLUDE_DIR=/usr/include
# ENV ARM_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_LIB_DIR=/usr/lib/arm-linux-gnueabihf
# ENV ARM_UNKNOWN_LINUX_GNUEABIHF_OPENSSL_INCLUDE_DIR=/usr/include