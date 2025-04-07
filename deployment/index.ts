import * as pulumi from "@pulumi/pulumi";
import * as k8s from "@pulumi/kubernetes";
import * as dotenv from "dotenv";

dotenv.config();

const config = new pulumi.Config();
const appName = "poem-dev";
const image = config.require("dockerImage");
const isMinikube = config.requireBoolean("isMinikube");
const passPhrase = pulumi.secret(process.env.PASSPHRASE || "");
const hmacSecret = pulumi.secret(process.env.HMAC_SECRET || "");
const logLevel = process.env.LOG_LEVEL || "info";

const ns = new k8s.core.v1.Namespace("poem-dev-ns", {
  metadata: { name: appName },
});

const secret = new k8s.core.v1.Secret("poem-dev-secret", {
  metadata: {
    name: "poem-dev-env",
    namespace: ns.metadata.name,
  },
  stringData: {
    PASSPHRASE: passPhrase,
    HMAC_SECRET: hmacSecret,
    LOG_LEVEL: logLevel,
  },
});

const appLabels = { app: appName };
const deployment = new k8s.apps.v1.Deployment("poem-dev-deploy", {
  metadata: {
    namespace: ns.metadata.name,
    name: "poem-dev",
  },
  spec: {
    selector: { matchLabels: appLabels },
    replicas: 1,
    template: {
      metadata: { labels: appLabels },
      spec: {
        containers: [
          {
            name: "poem-dev",
            image: image,
            ports: [{ containerPort: 8000 }],
            envFrom: [
              {
                secretRef: {
                  name: secret.metadata.name,
                },
              },
            ],
          },
        ],
      },
    },
  },
});

const service = new k8s.core.v1.Service("poem-dev-service", {
  metadata: {
    namespace: ns.metadata.name,
    name: "poem-dev-service",
  },
  spec: {
    type: "ClusterIP",
    selector: appLabels,
    ports: [
      {
        port: 80,
        targetPort: 8000,
      },
    ],
  },
});

export const ip = isMinikube
  ? service.spec.clusterIP
  : service.status.loadBalancer.apply(
      (lb) => lb.ingress[0].ip || lb.ingress[0].hostname,
    );
