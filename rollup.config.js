import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";
import { terser } from "rollup-plugin-terser";
import copy from 'rollup-plugin-copy';

const is_watch = !!process.env.ROLLUP_WATCH;

export default {
    input: {
        index: "./Cargo.toml",
    },
    output: {
        dir: "dist/js",
        format: "es",
        sourcemap: true,
    },
    plugins: [
        rust({
            extraArgs: {
              wasmOpt: [ "-Oz", "--enable-bulk-memory-opt", "--enable-nontrapping-float-to-int" ],
            },
        }),

        copy({
            targets: [
                { src: 'static/css/main.css', dest: 'dist/css/' },
                { src: 'static/img/favicon.ico', dest: 'dist/img/' },
                { src: 'static/img/burocratin.svg', dest: 'dist/img/' },
            ]
        }),

        is_watch && serve({
            contentBase: "dist",
            open: true,
        }),

        is_watch && livereload("dist"),

        !is_watch && terser(),
    ],
};
