FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive

# Install common dependencies and build tools
# libssl-dev and pkg-config are frequently needed for compiling common Rust crates
RUN apt-get update && apt-get install -y \
    curl \
    file \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    libzstd-dev \
    lldb \
    wget \
    software-properties-common \
    tree \
    gnupg \
    python3 \
    python3-pip \
    python3-venv \
    sudo \
    qemu-user \
    qemu-user-static \
    gcc-riscv64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

# Install LLVM 21 for Inkwell
RUN wget https://apt.llvm.org/llvm.sh && \
    chmod +x llvm.sh && \
    ./llvm.sh 21 all && \
    rm llvm.sh && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set LLVM environment variables for Inkwell
ENV LLVM_SYS_211_PREFIX="/usr/lib/llvm-21"

# Update PATH to include LLVM binaries
ENV PATH="/usr/lib/llvm-21/bin:${PATH}"

# Use the existing ubuntu user (UID 1000) and grant sudo access
RUN echo "ubuntu ALL=(root) NOPASSWD:ALL" > /etc/sudoers.d/ubuntu && \
    chmod 0440 /etc/sudoers.d/ubuntu

# Switch to ubuntu user for Rust installation
USER ubuntu
WORKDIR /home/ubuntu

# Install Rust using rustup for the user
# -y disables confirmation prompts
# --default-toolchain stable installs the stable release
# --profile default installs common components
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile default

# Update PATH so cargo and rustc are available
ENV PATH="/home/ubuntu/.cargo/bin:${PATH}"

# Install llvm-tools-preview component for llvm-cov support
RUN rustup component add llvm-tools-preview

# Install useful Rust components
RUN cargo install cargo-llvm-cov

# Verify installation
RUN rustc --version && cargo --version && python3 --version
