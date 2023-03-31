FROM docker/for-desktop-kernel:5.15.49-13422a825f833d125942948cf8a8688cef721ead AS kernel-headers
FROM debian:11

WORKDIR /
COPY --from=kernel-headers /*.tar ./
RUN tar -xvf kernel.tar & tar -xvf kernel-dev.tar & tar -xvf kernel-headers.tar

RUN echo deb http://cloudfront.debian.net/debian sid main >> /etc/apt/sources.list
RUN apt-get update
RUN apt-get install -y bpfcc-tools libbpfcc libbpfcc-dev

CMD ["/bin/sh"]
