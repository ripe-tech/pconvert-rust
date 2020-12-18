const assert = require("assert");

describe("Array", function() {
    describe("#indexOf()", function() {
        it("should return -1 when the value is not present", () => {
            assert.strictEqual(-1, [1, 2, 3].indexOf(4));
        });
    });
    describe("#length", function() {
        it("should return proper length", () => {
            assert.strictEqual(3, [1, 2, 3].length);
        });
    });
});
