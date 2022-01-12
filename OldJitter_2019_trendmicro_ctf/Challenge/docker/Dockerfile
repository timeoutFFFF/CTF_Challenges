FROM ubuntu:18.04

RUN sed -i "s/http:\/\/archive.ubuntu.com/http:\/\/mirror.csclub.uwaterloo.ca/g" /etc/apt/sources.list && \
    apt-get update && apt-get -y dist-upgrade && \
    apt-get install -y libicu-dev socat

RUN useradd -m ctf

WORKDIR /home/ctf

RUN cp -R /lib* /home/ctf && \
    cp -R /usr/lib* /home/ctf 


RUN mkdir /home/ctf/dev && \
    mknod /home/ctf/dev/null c 1 3 && \
    mknod /home/ctf/dev/zero c 1 5 && \
    mknod /home/ctf/dev/random c 1 8 && \
    mknod /home/ctf/dev/urandom c 1 9 && \
    chmod 666 /home/ctf/dev/*

RUN mkdir /home/ctf/bin && \
    cp /bin/sh /home/ctf/bin && \
    cp /bin/ls /home/ctf/bin && \
    cp /bin/cat /home/ctf/bin

COPY ./start.sh /start.sh
COPY ./tmp/myTmp /myTmp

#RUN echo "Blocked" > /etc/banner_fail

RUN chmod +x /start.sh
RUN chmod +x /myTmp

COPY ./bin/  /home/ctf/
#COPY ./tmp/ /home/ctf/bin

RUN  mkdir /home/ctf/tmp 

RUN chown -R root:ctf /home/ctf && \
    chmod -R 770 /home/ctf && \
    #chmod -R 670  /home/ctf/tmp && \
    chmod 740 /home/ctf/flag

CMD ["/start.sh"]
#CMD ["/usr/bin/watch -n 2 /home/ctf/bin/myTmp"]

EXPOSE 9999
