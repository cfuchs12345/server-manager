ARG UBUNTU_VERSION=focal
ARG VERSION=1
ARG NAME=server-manager-rust

FROM ubuntu:jammy
# need to repeat the variables after from since from consumes all args and they are not available afterwards
ARG HTTP_PORT

LABEL NAME ${NAME}
LABEL version ${VERSION}

ADD ../inet.dll ./
ADD ../inet.so ./
ADD ../server-manager-rust ./
ADD ../.env.example ./.env

ADD ../shipped_plugins ./
ADD ../server ./
ADD ../server-manager-frontend ./server/static
COPY /entrypoint.sh ./

RUN ls -altr


RUN apt update \
&& apt install -y openssh-server \
&& ssh-keygen -A \
&& chmod +x /server-manager-rust \
&& chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]

EXPOSE 0