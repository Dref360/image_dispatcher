# Image Dispatcher
The goal of this repo is to make a complete image data augmentation pipeline.

## How to run?
```bash
cargo run --release -- $FOLDER
```
where `$FOLDER` is a directory of images.

```
$FOLDER
  |-  1.png
  |-  2.png
  |-  ....
  |-  n.png
```

## Roadmap
- [x] API Design using transformers (similar to keras.Layers or torchvision.transforms)
- [ ] Add tests
- [x] Use rayon for async computing.
- [ ] Integrate with a ML framework such as TF or Torch.
