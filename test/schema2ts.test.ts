import { describe, expect, test } from "vitest";
import fs from "fs";
import path from "path";
import { schemaToTs, IOptions } from "../index";

const readFile = (pathToFind: string) =>
  fs.readFileSync(path.join(__dirname, pathToFind), "utf-8");

const expectCorrectOutput = (
  inputPath: string,
  outputPath: string,
  options?: IOptions
) => {
  const input = readFile(inputPath);
  const output = readFile(outputPath);
  expect(schemaToTs(input, options)).toBe(output);
};

describe("check whether output is correctly when use default config", () => {
  test("rustySchemaToTs parse an error schema", () => {
    expectCorrectOutput("./empty/input.json", "./empty/output.ts");
  });

  test("rustySchemaToTs parse correctly", () => {
    expectCorrectOutput("./test1/input.json", "./test1/output.ts");
  });

  test("rustySchemaToTs parse correctly with plain enum", () => {
    expectCorrectOutput("./test6/input.json", "./test6/output.ts");
  });

  test("rustySchemaToTs parse correctly with dulplicate type and auto increase", () => {
    expectCorrectOutput("./test7/input.json", "./test7/output.ts");
  });
});

describe("check whether output is correctly when change default config", () => {
  test("disable semi", () => {
    expectCorrectOutput("./test2/input.json", "./test2/output.ts", {
      semi: false,
      indent: 4,
      explain: "// This file is automatically generated by schema-ts",
    });
  });

  test("automatically generate comment", () => {
    expectCorrectOutput("./test3/input.json", "./test3/output.ts", {
      isGenComment: true,
    });
  });

  test("disable optional", () => {
    expectCorrectOutput("./test4/input.json", "./test4/output.ts", {
      optional: false,
      isGenComment: true,
    });
  });

  test("ignoreKeys", () => {
    expectCorrectOutput("./test5/input.json", "./test5/output.ts", {
      ignoreKeys: ["firstName", "obj", "hairColor", "arr"],
      isGenComment: true,
      optional: false,
    });
  });
});
