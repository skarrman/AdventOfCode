const { sortPackets } = require("../index");

test("sort", () => {
  expect(sortPackets([[]], [])).toStrictEqual(1);
});
