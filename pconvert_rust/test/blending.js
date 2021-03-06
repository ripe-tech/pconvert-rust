const fs = require("fs");
const path = require("path");
const assert = require("assert");

const pconvert = require("../../pkg/pconvert_rust.js");

const TEST_ASSETS = path.resolve("assets/test");

describe("NodeJS WASM", async function() {
    this.timeout(30000);

    describe("#getModuleConstants", function() {
        it("should have known module constants", () => {
            const constants = pconvert.getModuleConstants();
            const mandatoryKeys = [
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
            assert.deepStrictEqual(mandatoryKeys, Object.keys(constants));
        });
    });

    describe("#blendMultipleFs", function() {
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
            const algorithms = ["alpha", "multiplicative", "destination_over", "source_over"];
            pconvert.blendMultipleFs(paths, out, algorithm, algorithms, true);
            assert(fs.existsSync(out));
        });

        it("should blend multiple files from local file system with no algorithm specified", () => {
            const paths = [
                path.resolve(`${TEST_ASSETS}/sole.png`),
                path.resolve(`${TEST_ASSETS}/back.png`),
                path.resolve(`${TEST_ASSETS}/front.png`),
                path.resolve(`${TEST_ASSETS}/shoelace.png`),
                path.resolve(`${TEST_ASSETS}/background_alpha.png`)
            ];
            const out = path.resolve(`${TEST_ASSETS}/result_alpha_alpha_Fast_NoFilter.png`);
            const algorithm = null;
            const algorithms = null;
            pconvert.blendMultipleFs(paths, out, algorithm, algorithms, true);
            assert(fs.existsSync(out));
        });
    });

    describe("#blendMultipleFsAsync", function() {
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
            const algorithms = ["alpha", "multiplicative", "destination_over", "source_over"];
            await pconvert.blendMultipleFsAsync(paths, out, algorithm, algorithms, true);
            assert(fs.existsSync(out), `Expected final composition to be at ${out}`);
        });

        it("should asynchronously blend multiple files from local file system with no algorithm specified", async () => {
            const paths = [
                path.resolve(`${TEST_ASSETS}/sole.png`),
                path.resolve(`${TEST_ASSETS}/back.png`),
                path.resolve(`${TEST_ASSETS}/front.png`),
                path.resolve(`${TEST_ASSETS}/shoelace.png`),
                path.resolve(`${TEST_ASSETS}/background_alpha.png`)
            ];
            const out = path.resolve(`${TEST_ASSETS}/result_alpha_alpha_Fast_NoFilter.png`);
            const algorithm = null;
            const algorithms = null;
            await pconvert.blendMultipleFsAsync(paths, out, algorithm, algorithms, true);
            assert(fs.existsSync(out), `Expected final composition to be at ${out}`);
        });
    });
});
