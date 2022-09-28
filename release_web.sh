wasm-pack build --target web
rm -r dist-web /s /q 2>nul
mkdir dist-web

cp -r pkg/*.js dist-web/pkg/
cp -r pkg/*.wasm dist-web/pkg/
cp -r pkg/*.d.ts dist-web/pkg/
cp -r pkg/*.wasm.d.ts dist-web/pkg/

cp -r assets/*.* dist-web/assets/