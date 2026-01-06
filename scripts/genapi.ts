import { execSync } from "node:child_process";
import { existsSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";

const OPENAPI_URL = "http://127.0.0.1:3000/api-docs/openapi.json";
const GIT_ROOT = execSync("git rev-parse --show-toplevel").toString().trim();
const OUTPUT_DIR = resolve(GIT_ROOT, "packages/reqx");
console.log(OUTPUT_DIR);

async function generateAPIClient(): Promise<void> {
  try {
    // Ensure output directory exists
    if (!existsSync(OUTPUT_DIR)) {
      mkdirSync(OUTPUT_DIR, { recursive: true });
    }

    console.log(`Generating TypeScript Axios client from ${OPENAPI_URL}...`);

    // Run openapi-generator with typescript-axios generator
    const command = [
      "npx @openapitools/openapi-generator-cli generate",
      `-i ${OPENAPI_URL}`,
      `-g typescript-axios`,
      `-o ${OUTPUT_DIR}`,
      "--skip-validate-spec",
      "--additional-properties=npmVersion=1.0.0,npmName=@vexillum/reqx,supportsES6=true,enumPropertyNaming=original",
    ].join(" ");

    execSync(command, { stdio: "inherit" });

    console.log(`✓ API client generated successfully at ${OUTPUT_DIR}`);
  } catch (error) {
    console.error(
      "Error generating API client:",
      error instanceof Error ? error.message : error
    );
    process.exit(1);
  }
}

generateAPIClient();
