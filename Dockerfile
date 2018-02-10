FROM ubuntu # wouldn't work with alpine
ENV LANG C.UTF-8
RUN apt-get update && apt-get -qqqy install curl
RUN cd
RUN curl https://sh.rustup.rs -sSf > installer.sh
RUN bash ./installer.sh -y

WORKDIR /root/rusty

CMD ["bash"]
