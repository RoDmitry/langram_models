# Langram models

You can download this repository, and remove some models ngrams locally (so that the final binary would be lighter), but remember that an ngram model len must be the same for all of the languages of the script (`Hans`, `Hant`, `Kore`, `Jpan` must be considered as the same script).

```
git clone https://github.com/RoDmitry/langram_models.git
# set langram version in Cargo.toml
# for big-endian targets enable `rkyv::big_endian` feature
cargo run --release
```

### Sources

Trained on [OpenLID](https://github.com/laurieburchell/open-lid-dataset) (201 languages).
