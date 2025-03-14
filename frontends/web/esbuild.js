const esbuild = require('esbuild');
const cssModulesPlugin = require('esbuild-css-modules-plugin');
const dotenv = require("dotenv");
const { argv } = require("process");


// Get cli args
const args = argv.slice(2);
const isWatchMode = args.includes("--watch");

// Load env variables from .env
const envResult = dotenv.config();
if (envResult.error) {
    throw new Error(`Error loading .env file: ${envResult.error}`);
}

// env checks
if (!process.env.BASE_FRONTEND_URL_DEV) {
    throw new Error("BASE_FRONTEND_URL_DEV not set");
}
if (!process.env.BASE_FRONTEND_URL_PROD) {
    throw new Error("BASE_FRONTEND_URL_PROD not set");
}
if (!process.env.BASE_BACKEND_URL_DEV) {
  throw new Error("BASE_BACKEND_URL_DEV not set");
}
if (!process.env.BASE_BACKEND_URL_PROD) {
  throw new Error("BASE_BACKEND_URL_PROD not set");
}

// Setting Urls
const isProduction = process.env.PRODUCTION?.toLowerCase() === "true";

const baseBackendUrl = isProduction
    ? process.env.BASE_BACKEND_URL_PROD
    : process.env.BASE_BACKEND_URL_DEV;

const baseFrontendUrl = isProduction
    ? (process.env.BASE_FRONTEND_URL_PROD)
    : (process.env.BASE_FRONTEND_URL_DEV);


async function build() {   
    const buildOpts = {
        plugins: [
          cssModulesPlugin()
        ],
        entryPoints: ['src/index.tsx'],
        bundle: true,
        outfile: 'public/bundle.js',
        format: 'esm',
        define: {
          'process.env.NODE_ENV': JSON.stringify(isProduction ? "production" : "development"),
          'process.env.PRODUCTION': JSON.stringify(process.env.PRODUCTION),
          'process.env.BASE_BACKEND_URL': JSON.stringify(baseBackendUrl),
          'process.env.BASE_FRONTEND_URL': JSON.stringify(baseFrontendUrl),
        },
        minify: true,
        sourcemap: true,
        loader: {
          '.js': 'jsx',
          '.tsx': 'tsx',
          '.ts': 'ts',
          '.wasm': 'binary',
          '.css': 'css',
          ".gif": "file",
          ".jpg": "file",
          ".png": "file",
        },
    }

    if (isWatchMode) {
        console.info("Watching for changes... (use with serve)");
        const ctx = await esbuild.context(buildOpts);
        return new Promise(async (res) => {
            await ctx.watch();
        });
    } else {
        await esbuild.build(buildOpts);
    }
}


build()
    .then(() => {
        console.log("Build completed successfully");
        process.exit(0);
    })
    .catch((error) => {
        console.error("Build failed:", error);
        process.exit(1);
    });
  