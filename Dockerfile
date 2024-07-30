# Use an official Ubuntu as a parent image
FROM rust:1.80

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update
RUN apt install -y protobuf-compiler
ARG USER_NAME
ARG USER_ID
ARG GROUP_ID
# Update the package lists
ENV USER_HOME=/home/${USER_NAME}

RUN groupadd -g ${GROUP_ID} ${USER_NAME}
RUN useradd -u ${USER_ID} -g ${USER_NAME} -m -d ${USER_HOME} ${USER_NAME}
USER ${USER_NAME}
WORKDIR ${USER_HOME}
# Expose the port on which your Drogon application will run
EXPOSE 9000
EXPOSE 5000



