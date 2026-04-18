const fs = require("fs");
const path = require("path");

const ESP_OK_CODE = 0;
const RUST_LIB_PATH = path.join(__dirname, "..", "src", "lib.rs");
const ERROR_FILE_PATH = path.join(__dirname, "errors.txt");

const TOK_ESP_IDF_ERROR_CODE_LIST = "/* ESP_IDF_ERROR_CODE_LIST */";
const TOK_ESP_IDF_ERROR_CODE_MATCH_ARM = "/* ESP_IDF_ERROR_CODE_MATCH_ARM */"
const TOK_ESP_IDF_ERROR_NAME_LOOKUP_TABLE = "/* ESP_IDF_ERROR_NAME_LOOKUP_TABLE */";
const TOK_ESP_IDF_ERROR_INFO_LOOKUP_TABLE = "/* ESP_IDF_ERROR_INFO_LOOKUP_TABLE */";

const ESP_IDF_ERROR_CODES_LINE = "pub const ESP_IDF_ERROR_CODES: [i32; %SIZE%] = [%ELEMENTS%];"

function parseErrorInfoLine(input) {
    input = input
      .replace(":", "")
      .split(" ");
    let [name, code, ...desc] = input;
    desc = desc.join(" ");
    code = code.replace(/\(|\)/g, "");
    return [name, code, desc];
}

const prevSourceCode = fs.readFileSync(RUST_LIB_PATH, "utf8");
const fullErrorInfo = fs.readFileSync(ERROR_FILE_PATH, "utf-8")
    .split("\r\n")
    .filter(l => l.length > 2)
    .map(parseErrorInfoLine)
    // Filter out the success exit code
    // and base error codes which always start with two zeroes
    // these base error codes define offsets for specific types of errors
    // (e.g ESP_ERR_ULP_BASE (=0x1200) as the offset for all ULP-related error codes)
    .filter(([_a, code, _b]) => code != ESP_OK_CODE && (code & 0xFF) !== 0);

const errorCodes = fullErrorInfo.map(info => info[1]);
const errorCodesLength = errorCodes.length;

function findSectionIndentation(sourceCode, sectionDelimitToken) {
    return sourceCode
      .split("\n")
      .find(l => l.includes(sectionDelimitToken))
      .split(sectionDelimitToken)[0]
}

const nameLookupTable = fullErrorInfo.map(([name, code, _]) => `${code} => "${name}"`);
const infoLookupTable = fullErrorInfo.map(([name, code, desc]) => `${code} => "[${name}]: ${desc || "No further error description"}"`);

function replaceDelimitedSection(sourceCode, sectionDelimitToken, replacement, isLookupTable) {
    const sectionIndent = findSectionIndentation(sourceCode, sectionDelimitToken);
    sourceCode = sourceCode.split("\n");
    const firstDelimitTokenIdx = sourceCode.findIndex(line => line.includes(sectionDelimitToken)) + 1;
    const lastDelimitTokenIdx = sourceCode.findLastIndex(line => line.includes(sectionDelimitToken));

    const firstSourceCodePart = sourceCode.slice(0, firstDelimitTokenIdx).join("\n");
    const lastSourceCodePart = sourceCode.slice(lastDelimitTokenIdx, sourceCode.length).join("\n");

    const replacementLines = replacement.map(l => sectionIndent + l).join(",\n") + (replacement.length > 1 ? "," : "");
    const replacedCode = [firstSourceCodePart, replacementLines, lastSourceCodePart];

    return replacedCode.join("\n");
}

const filledErrorCodesLine =
    ESP_IDF_ERROR_CODES_LINE
      .replace("%SIZE%", errorCodesLength)
      .replace("%ELEMENTS%", errorCodes.join(", "));
const errorCodesMatchArm = errorCodes.join(" | ");

const firstPass = replaceDelimitedSection(prevSourceCode, TOK_ESP_IDF_ERROR_CODE_LIST, [filledErrorCodesLine]);
const secondPass = replaceDelimitedSection(firstPass, TOK_ESP_IDF_ERROR_CODE_MATCH_ARM, [errorCodesMatchArm]);
const thirdPass = replaceDelimitedSection(secondPass, TOK_ESP_IDF_ERROR_NAME_LOOKUP_TABLE, nameLookupTable);
const fourthPass = replaceDelimitedSection(thirdPass, TOK_ESP_IDF_ERROR_INFO_LOOKUP_TABLE, infoLookupTable);
fs.writeFileSync(RUST_LIB_PATH, fourthPass, "utf8");