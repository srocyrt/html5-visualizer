import esbuild from "esbuild";
import { wasmLoader } from 'esbuild-plugin-wasm';
import htmlPlugin from '@chialab/esbuild-plugin-html';

let ctx = await esbuild.context({
    entryPoints: ['./www/index.html'],
    format: 'esm',
    bundle: true,
    outdir: 'dist',
    supported: {
        'top-level-await': true,
    },
    plugins: [
        wasmLoader(),
        htmlPlugin(),
    ],
});

ctx.serve({
    host: "localhost",
    port: 8080,
    servedir: './dist',
})