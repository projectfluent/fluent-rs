#!/usr/bin/env node

'use strict';
const fs = require('fs');

fs.readdirSync("./").forEach(file => {
  if (file.endsWith(".json")) {
    fs.readFile(file, convert.bind(this, file));
  }
})

function write(s, fileName) {
  const data = new Uint8Array(Buffer.from(s));
  fs.writeFile(fileName, data, (err) => {
    if (err) throw err;
    console.log(`The file "${fileName}" has been saved!`);
  });
}


function convert(fileName, err, data) {
  let ast = JSON.parse(data.toString());

  remove_leading_dash_from_term(ast);
  remove_unambigous_types(ast);
  split_multiline_text_elements(ast);

  let s = JSON.stringify(ast, null, 4);
  write(s, fileName);
}

function remove_leading_dash_from_term(ast) {
  for (let i in ast) {
    let node = ast[i];
    if (Array.isArray(node)) {
      remove_leading_dash_from_term(node);
    } else if (typeof node === "object") {
      remove_leading_dash_from_term(node);
    } else if (i === "name" &&
      typeof node == "string" &&
      node.startsWith("-")) {
      ast[i] = node.substr(1);
    }
  }
}

function remove_unambigous_types(ast, parent_node = null, parent_key = null) {
  for (let i in ast) {
    let node = ast[i];
    if (Array.isArray(node)) {
      remove_unambigous_types(node, ast);
    } else if (typeof node === "object") {
      remove_unambigous_types(node, ast, i);
    } else if (i === "type" &&
      parent_key !== "selector" &&
      ["Resource",
       "Pattern",
       "Function",
       "Variant",
       "SelectExpression",
       "Attribute",
       "NamedArgument",
       "VariantList",
       "Identifier"].includes(node)) {
      ast[i] = undefined;
    } else if (parent_key == "key" &&
     ["NumberLiteral"].includes(node)) {
      ast[i] = undefined;
    }
  }
}

function split_multiline_text_elements(ast) {
  for (let i in ast) {
    let node = ast[i];
    if (Array.isArray(node)) {
      split_multiline_text_elements(node);
    } else if (typeof node === "object" &&
      node !== null &&
      node["type"] === "TextElement") {
      let parts = node["value"].split("\n");
      let elements = parts.filter(v => v != "").map((v, i) => {
        let last = parts.length - 1 === i;
        return {
          "type": "TextElement",
          "value": last ? v : `${v}\n`
        };
      });
      ast = ast.splice(i, 1, ...elements);
    } else if (typeof node === "object") {
      split_multiline_text_elements(node);
    }
  }
}
