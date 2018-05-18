FROM rust:1.25.0
RUN apt-get update
ADD cmake-3.11.1.tar.gz /tmp
WORKDIR /tmp/cmake-3.11.1
RUN ./bootstrap
RUN make && make install
WORKDIR /home
RUN apt-get install -y git
RUN git clone https://github.com/google/leveldb
WORKDIR /home/leveldb/build
RUN cmake -DCMAKE_BUILD_TYPE=Debug ..
RUN cmake --build . --target install
# update registry index
RUN cargo search
WORKDIR /home
