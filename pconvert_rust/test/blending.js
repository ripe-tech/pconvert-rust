const assert = require("assert");
const pconvert = require("pconvert-rust");

describe("NodeJS WASM", async function() {
    it("should get module constants with specific keys", () => {
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
        Object.keys(constants).forEach(k => keys.includes(k));
    });
});
