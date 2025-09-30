// build.ts
import { basename, dirname, join, resolve } from "path";
import { existsSync, mkdirSync } from "fs";

async function build() {
  const inputFile = process.argv[2];

  if (!inputFile) {
    console.error("❌ Usage: bun run build <file-path>");
    console.error("   Example: bun run build src/9085_v1.ts");
    process.exit(1);
  }

  const absolutePath = resolve(inputFile);

  if (!existsSync(absolutePath)) {
    console.error(`❌ File not found: ${inputFile}`);
    process.exit(1);
  }

  const fileName = basename(inputFile, ".ts");
  const outputDir = "dist";
  const outputFile = join(outputDir, `${fileName}.js`);

  // dist 폴더가 없으면 생성
  if (!existsSync(outputDir)) {
    mkdirSync(outputDir, { recursive: true });
  }

  console.log(`🔨 Building ${inputFile}...`);

  try {
    await Bun.build({
      entrypoints: [absolutePath],
      outdir: outputDir,
      target: "node",
      format: "cjs",
      minify: false,
      sourcemap: "external",
      splitting: false,
      naming: `${fileName}.js`,
    });

    console.log(`✅ Build completed: ${outputFile}`);
  } catch (error) {
    console.error("❌ Build failed:", error);
    process.exit(1);
  }
}

build();
