const fs = require("fs");
const path = require("path");

const ESP_OK_CODE = 0;

const RUST_LIB_PATH = path.join(__dirname, "..", "src", "lib.rs");
const ERROR_FILE_PATH = path.join(__dirname, "errors.txt");

const TOK_ESP_IDF_ERROR_COUNT = "/* ESP_IDF_ERROR_COUNT */";
const ESP_IDF_ERROR_COUNT_LINE = "pub const ESP_IDF_ERROR_COUNT: usize = %SIZE%;";

const TOK_ESP_IDF_ERROR_CODE_LIST = "/* ESP_IDF_ERROR_CODE_LIST */";
const ESP_IDF_ERROR_CODES_LINE = "pub const ESP_IDF_ERROR_CODES: [i32; ESP_IDF_ERROR_COUNT] = [%ELEMENTS%];"

const TOK_ESP_IDF_ERROR_ENUM = "/* ESP_IDF_ERROR_ENUM */";

const TOK_ESP_IDF_ERROR_NAME_LOOKUP_TABLE = "/* ESP_IDF_ERROR_NAME_LOOKUP_TABLE */";
const TOK_ESP_IDF_ERROR_INFO_LOOKUP_TABLE = "/* ESP_IDF_ERROR_INFO_LOOKUP_TABLE */";

const TOK_ESP_IDF_ERROR_FROM_CODE_LOOKUP_TABLE = "/* ESP_IDF_ERROR_FROM_CODE_LOOKUP_TABLE */";
const TOK_ESP_IDF_CODE_FROM_ERROR_LOOKUP_TABLE = "/* ESP_IDF_CODE_FROM_ERROR_LOOKUP_TABLE */";



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

const errorCodes = fullErrorInfo.map(([_a, code, _b]) => code);
const errorCodesLength = errorCodes.length;

function findSectionIndentation(sourceCode, sectionDelimitToken) {
    console.log(sectionDelimitToken);
    return sourceCode
      .split("\n")
      .find(l => l.includes(sectionDelimitToken))
      .split(sectionDelimitToken)[0]
}

const errorCodeLookupTable = fullErrorInfo.map(([name, code, _]) => `${code} => Some(Self::${name})`);
const codeErrorLookupTable = fullErrorInfo.map(([name, code, _]) => `Self::${name} => ${code}`);

const errorNameLookupTable = fullErrorInfo.map(([name, code, desc ]) => `Self::${name} => "${name}"`);
const errorInfoLookupTable = fullErrorInfo.map(([name, code, desc ]) => `Self::${name} => "[${name}]: ${desc || "No further error description"}"`);

const codeEnum = fullErrorInfo.map(([name, code, _]) => `${name} = ${code}`);

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
const filledErrorCountLine =
    ESP_IDF_ERROR_COUNT_LINE
        .replace("%SIZE%", errorCodesLength);


const processPasses = [
    [TOK_ESP_IDF_ERROR_COUNT, [filledErrorCountLine]],
    [TOK_ESP_IDF_ERROR_CODE_LIST, [filledErrorCodesLine]],
    [TOK_ESP_IDF_ERROR_ENUM, codeEnum],
    [TOK_ESP_IDF_ERROR_FROM_CODE_LOOKUP_TABLE, errorCodeLookupTable],
    [TOK_ESP_IDF_CODE_FROM_ERROR_LOOKUP_TABLE, codeErrorLookupTable],
    [TOK_ESP_IDF_ERROR_NAME_LOOKUP_TABLE, errorNameLookupTable],
    [TOK_ESP_IDF_ERROR_INFO_LOOKUP_TABLE, errorInfoLookupTable]
];

let codeBuffer = prevSourceCode;

for(let pass of processPasses) {
    codeBuffer = replaceDelimitedSection(codeBuffer, pass[0], pass[1]);
}

console.log(codeBuffer);
fs.writeFileSync(RUST_LIB_PATH, codeBuffer, "utf8");