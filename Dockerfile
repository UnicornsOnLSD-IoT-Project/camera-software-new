# Use the container that comes with `cross` as a base. It's already got
# a cross-compile toolchain installed, so that's less work for us.
FROM rustembedded/cross:arm-unknown-linux-gnueabihf-0.2.1

ENV TZ=Europe/London
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update
RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get upgrade --assume-yes && \
    apt-get install --assume-yes wget && \
    apt-get download libopencv-dev:armhf llvm clang libclang-10-dev:armhf libopencv-calib3d-dev:armhf libopencv-contrib-dev:armhf libopencv-features2d-dev:armhf libopencv-highgui-dev:armhf libopencv-imgcodecs-dev:armhf libopencv-objdetect-dev:armhf libopencv-stitching-dev:armhf libopencv-superres-dev:armhf libopencv-videoio-dev:armhf libopencv-videostab-dev:armhf libopencv-viz-dev:armhf libopencv-calib3d3.2:armhf libopencv-contrib3.2:armhf libopencv-features2d3.2:armhf libopencv-highgui3.2:armhf libopencv-imgcodecs3.2:armhf libopencv-videoio3.2:armhf

RUN wget "http://192.168.0.10:8000/libraspberrypi-dev_1.20210108-1_armhf.deb"
RUN wget "http://192.168.0.10:8000/libraspberrypi0_1.20210108-1_armhf.deb"
RUN wget "http://192.168.0.10:8000/raspberrypi-bootloader_1.20210108-1_armhf.deb"

# RUN dpkg -i raspberrypi-bootloader_1.20210108-1_armhf.deb
# RUN dpkg -i libraspberrypi0_1.20210108-1_armhf.deb
# RUN dpkg -i libraspberrypi-dev_1.20210108-1_armhf.deb 
RUN dpkg -i  *.deb