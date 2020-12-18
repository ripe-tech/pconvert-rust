const fs = require("fs");
const path = require("path");
const assert = require("assert");
const pconvert = require("pconvert-rust");

const TEST_ASSETS = path.resolve("assets/test");

describe("NodeJS WASM", async function() {
    it("should have known module constants", () => {
        const constants = pconvert.getModuleConstants();
        const keys = [
            "ALGORITHMS",
            "COMPILATION_DATE",
            "COMPILATION_TIME",
            "COMPILER",
            "COMPILER_VERSION",
            "COMPRESSION_TYPES",
            "FEATURES",
            "FILTER_TYPES",
            "LIBPNG_VERSION",
            "PLATFORM_CPU_BITS",
            "VERSION"
        ];
        keys.forEach(k => assert(k in constants));
        Object.keys(constants).forEach(k => assert(keys.includes(k)));
    });

    it("should blend multiple files from local file system", () => {
        const paths = [
            path.resolve(`${TEST_ASSETS}/sole.png`),
            path.resolve(`${TEST_ASSETS}/back.png`),
            path.resolve(`${TEST_ASSETS}/front.png`),
            path.resolve(`${TEST_ASSETS}/shoelace.png`),
            path.resolve(`${TEST_ASSETS}/background_alpha.png`)
        ];
        const out = path.resolve(`${TEST_ASSETS}/result_alpha_alpha_Fast_NoFilter.png`);
        const algorithm = "alpha";
        const algorithms = null;
        pconvert.blendMultipleFs(paths, out, algorithm, algorithms, true);
        assert(fs.existsSync(out));
    });

    it("should asynchronously blend multiple files from local file system", async () => {
        const paths = [
            path.resolve(`${TEST_ASSETS}/sole.png`),
            path.resolve(`${TEST_ASSETS}/back.png`),
            path.resolve(`${TEST_ASSETS}/front.png`),
            path.resolve(`${TEST_ASSETS}/shoelace.png`),
            path.resolve(`${TEST_ASSETS}/background_alpha.png`)
        ];
        const out = path.resolve(`${TEST_ASSETS}/result_alpha_alpha_Fast_NoFilter.png`);
        const algorithm = "alpha";
        const algorithms = null;
        await pconvert.blendMultipleFsAsync(paths, out, algorithm, algorithms, true);
        assert(fs.existsSync(out));
    });
});
