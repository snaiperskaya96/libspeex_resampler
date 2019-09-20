Literally just

```bash
sudo apt install libspeexdsp-dev
bindgen /usr/include/speex/speex_resampler.h -o src/bindings.rs
```

Check [speex documentation](https://www.speex.org/docs/manual/speex-manual/node7.html#SECTION00760000000000000000).