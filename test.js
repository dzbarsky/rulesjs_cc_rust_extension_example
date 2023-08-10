const {demo} = require("./bazel-bin/index.node");
console.log(Buffer.from(demo()).toString());
