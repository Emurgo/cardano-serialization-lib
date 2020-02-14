module.exports = {
  spec: "src/**/*.spec.ts",
  require: ["ts-node/register", "source-map-support/register"],
  recursive: true,
  "watch-extensions": ['ts'],
  exit: true,
};
