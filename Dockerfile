# Stage 1: Build Rust WASM
FROM rust:1.83-slim as rust-builder

# Install wasm-pack
RUN cargo install wasm-pack

WORKDIR /app

# Copy Rust sources
COPY rust-libs ./rust-libs

# Build WASM modules
# We need to mirror the structure expected by package.json scripts or run commands directly
# The scripts are: wasm-pack build rust-libs/calculator --target web
RUN wasm-pack build rust-libs/calculator --target web
RUN wasm-pack build rust-libs/snake --target web

# Stage 2: Build Node.js Frontend
FROM node:20-slim as node-builder

WORKDIR /app

# Copy package files
COPY package.json package-lock.json ./

# Install dependencies
RUN npm ci

# Copy only the pkg directories from rust-builder
# Vite config expects: @calculator -> /rust-libs/calculator/pkg
# So we need to reconstruct that path structure
COPY --from=rust-builder /app/rust-libs/calculator/pkg ./rust-libs/calculator/pkg
COPY --from=rust-builder /app/rust-libs/snake/pkg ./rust-libs/snake/pkg

# Copy the rest of the source code
COPY . .

# Build the frontend (skip build:wasm since we already brought artifacts)
# We override the build script or just run vite build directly to avoid re-running wasm-pack (which isn't installed here)
RUN npx tsc -b && npx vite build

# Stage 3: Serve with Nginx
FROM nginx:alpine

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=node-builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
