# Getting Started

## How to build and run the project

### Local Build

From root folder, run

```bash
cargo run
```

### Build via Docker

From root folder, run to build Docker image

```bash
docker build -t poem_dev_take_home:local .
```

To run Docker image, run

```bash
docker run -p 8000:8000 poem_dev_take_home:local
```

## Kubernetes Deployment with Pulumi & Minikube

First, let's get Minikube up and running. If you don't have minikube installed, check this [link](https://minikube.sigs.k8s.io/docs/start/?arch=%2Fmacos%2Farm64%2Fstable%2Fbinary+download)

```bash
minikube start
```

Minikube has a decent UI to view

```bash
minikube dashboard
```

To deploy the app on Pulumi, we'd be using Kubernetes

To get started let's build our Docker image

```bash
eval $(minikube docker-env)  # Point your shell to Minikube's Docker daemon
docker build -t poem_dev_take_home:local .
```

then deploy to Minikube

```bash
kubectl create deployment poem-dev-take-home --image=poem_dev_take_home:local  # Attach Docker image
```

### Push to Pulumi

To push to Pulumi, you would need to have an account on Pulumi.

#### Deploy

To start, make sure you have Pulumi cli [installed](https://www.pulumi.com/docs/iac/download-install/). Change directory to `deployment`

```bash
cd deployment
```

Once in, let's do a few setup

```bash
pulumi config set poem_dev_take_home:local poem_dev_take_home:local

pulumi config set poem_dev:isMinikube true
```

Once setup, run

```bash
pulumi up
```

## JWT

We have a simple JWT implementation, a JWK is issued once logged in and required for `/protected` route.

an `HMAC_SECRET` is required in `.env` file. The secret should be generated off the SHA-256 Algorithm

## Environment Variables

For the program to run you need two environment variables

- `HMAC_SECRET` - The secret used to sign JWT tokens
- `LOG_LEVEL` - Log level for the application

### For example
```bash
HMAC_SECRET = "C812BEEC03597FE788771212141CD4EB96E9D1C2D763E8651DEFC38453F54EF7" # Only for testing
LOG_LEVEL = "debug"
```
