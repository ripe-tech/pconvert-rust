const path = require("path");
module.exports = {
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "index.js"
    },
    mode: "development",
    devServer: {
        port: process.env.PORT ? parseInt(process.env.PORT) : 3000
    }
};
