FROM ragnaroek/rust-raspberry:1.43.1
USER root
RUN apt-get -y install wget build-essential libasound2-dev libc6-dev-armhf-cross
RUN wget http://www.portaudio.com/archives/pa_stable_v190600_20161030.tgz
RUN tar -xvzf pa_stable_v190600_20161030.tgz
ENV CC=/home/cross/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc
ENV CXX=/home/cross/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-g++
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH=/home/cross/project/target/arm-unknown-linux-gnueabihf/release/build/portaudio-62a8f1ce1e6e6237/out/lib/pkgconfig
RUN ls /home/cross/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin
#ENV PORTAUDIO_ONLY_STATIC=1
RUN cd portaudio && CC=/home/cross/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-gcc CXX=/home/cross/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/arm-linux-gnueabihf-g++ ./configure --target=arm-none-linux-gnueabi --host=arm-linux-gnueabihf && make ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf-
RUN cd portaudio && make install
USER cross
RUN ls /usr/lib
ENV RUST_BACKTRACE=1
#ENTRYPOINT [""]
