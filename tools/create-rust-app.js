#!/usr/bin/env node

const fs = require("fs");
const path = require("path");

function copyDir(src, dest, applyReplacements) {
  if (!fs.existsSync(dest)) {
    fs.mkdirSync(dest, { recursive: true });
  }

  const entries = fs.readdirSync(src, { withFileTypes: true });

  for (let entry of entries) {
    const srcPath = path.join(src, entry.name);
    const destPath = path.join(dest, entry.name);

    if (entry.isDirectory()) {
      copyDir(srcPath, destPath, applyReplacements);
    } else {
      let content = fs.readFileSync(srcPath, "utf8");

      content = applyReplacements(content, srcPath);

      fs.writeFileSync(destPath, content);
    }
  }
}

function validateAppName(appName) {
  if (!appName) {
    console.error(
      "❌ Please provide an app name: npm run create:rust-app <app-name>"
    );
    process.exit(1);
  }

  if (!/^[a-zA-Z][a-zA-Z0-9_]*$/.test(appName)) {
    console.error(
      "❌ App name must start with a letter and contain only letters, numbers, and underscores (no hyphens)"
    );
    process.exit(1);
  }

  if (appName.includes("-")) {
    console.error(
      "❌ Hyphens are not allowed in Rust app names. Use underscores instead."
    );
    console.error(`💡 Try: ${appName.replace(/-/g, "_")}`);
    process.exit(1);
  }

  if (appName.length > 50) {
    console.error("❌ App name must be 50 characters or less");
    process.exit(1);
  }

  return true;
}

function createRustApp(appName) {
  validateAppName(appName);

  const templateDir = "tools/rust_app_template";
  const targetDir = `apps/${appName}`;

  if (!fs.existsSync(templateDir)) {
    console.error(`❌ Template directory not found: ${templateDir}`);
    console.error(
      "💡 Make sure the rust app template exists in tools/templates/"
    );
    process.exit(1);
  }

  if (fs.existsSync(targetDir)) {
    console.error(`❌ App ${appName} already exists!`);
    process.exit(1);
  }

  console.log(`🦀 Creating Rust app: ${appName}`);

  function applyReplacements(content) {
    console.log("ini content: ", content);
    return content.replace(/rust_app_template/g, appName);
  }

  copyDir(templateDir, targetDir, applyReplacements);

  const cargoTomlPath = "Cargo.toml";
  let cargoContent = fs.readFileSync(cargoTomlPath, "utf8");

  const memberLine = `\t'${targetDir}',`;
  if (!cargoContent.includes(memberLine)) {
    cargoContent = cargoContent.replace(
      /members = \[\s*\n/,
      `members = [\n${memberLine}\n`
    );
    fs.writeFileSync(cargoTomlPath, cargoContent);
  }

  console.log(`✅ Successfully created Rust app: ${appName}`);
  console.log(`📁 Location: ${targetDir}`);
  console.log(`🚀 To build: nx build ${appName}`);
  console.log(`🧪 To test: nx test ${appName}`);
  console.log(`▶️  To run: nx run ${appName}`);
}

const appName = process.argv[2];
createRustApp(appName);
