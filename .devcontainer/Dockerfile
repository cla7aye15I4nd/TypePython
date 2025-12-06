FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive

# Install common dependencies and build tools
# libssl-dev and pkg-config are frequently needed for compiling common Rust crates
RUN apt-get update && apt-get install -y \
    curl \
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

# Install Rust using rustup
# -y disables confirmation prompts
# --default-toolchain stable installs the stable release
# --profile minimal keeps the size down (you can add components later)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile default

# Update PATH so cargo and rustc are available
ENV PATH="/root/.cargo/bin:${PATH}"

# Verify installation
RUN rustc --version && cargo --version && python3 --version