# Stage 1: Build
FROM node:20-alpine AS build
WORKDIR /app

# Copy dependency files first for layer caching
COPY package.json package-lock.json ./
RUN npm ci

# Copy source and build
COPY . .
ARG VITE_API_BASE_URL=""
RUN npm run build

# Stage 2: Runtime
FROM nginx:alpine AS runtime

# SPA routing config: redirect all non-file requests to index.html
RUN printf 'server {\n\
    listen 80;\n\
    server_name _;\n\
    root /usr/share/nginx/html;\n\
    index index.html;\n\
\n\
    location / {\n\
        try_files $uri $uri/ /index.html;\n\
    }\n\
\n\
    location /api/ {\n\
        proxy_pass http://backend:8080;\n\
        proxy_set_header Host $host;\n\
        proxy_set_header X-Real-IP $remote_addr;\n\
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;\n\
        proxy_set_header X-Forwarded-Proto $scheme;\n\
    }\n\
\n\
    location /static/ {\n\
        proxy_pass http://backend:8080;\n\
        proxy_set_header Host $host;\n\
        proxy_set_header X-Real-IP $remote_addr;\n\
    }\n\
\n\
    location /swagger-ui/ {\n\
        proxy_pass http://backend:8080;\n\
        proxy_set_header Host $host;\n\
    }\n\
\n\
    location /api-docs/ {\n\
        proxy_pass http://backend:8080;\n\
        proxy_set_header Host $host;\n\
    }\n\
}\n' > /etc/nginx/conf.d/default.conf

COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
