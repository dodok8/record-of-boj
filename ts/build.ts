import { join, relative } from "path";

const srcDir = "./src";
const outDir = "./dist";

async function buildFile(filePath: string) {
  const fullPath = join(srcDir, filePath);
  const relativeFile = relative(srcDir, fullPath);
  const outFile = join(outDir, relativeFile).replace(/\.ts$/, ".js");

  try {
    await Bun.build({
      entrypoints: [fullPath],
      outdir: outDir,
      minify: false,
      target: "node",
      sourcemap: "none",
    });
    console.log(`Built: ${fullPath} -> ${outFile}`);
  } catch (error) {
    console.error(`Error building ${fullPath}:`, error);
  }
}

async function main() {
  const filesToBuild = Bun.argv.slice(2); // Get command line arguments, excluding 'bun' and the script name

  if (filesToBuild.length === 0) {
    console.log(
      "Please provide file names to build. Usage: bun run build.ts file1.ts file2.ts ..."
    );
    return;
  }

  for (const file of filesToBuild) {
    await buildFile(file);
  }
}

main().catch(console.error);

export {};
