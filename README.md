# Detection Color

Calculating the optimal color for bounding boxes in object detection.

## Introduction

Detection models are identify objects in images. Visualizing the results of these models is important for understanding the model's performance. One way to visualize the results is to draw bounding boxes around the detected objects. The color of the bounding box can be used to indicate the confidence of the detection or the class of the object. This package provides a way to calculate the optimal color for bounding boxes.

## Methodology

Utilising a Camera Trap Dataset, we take the mean RGB values of the pixels 1px within and 1px outside of a 1px bounding box around each object. We then calculate the color with the highest contrast ratio to the background color. This is the optimal color for the bounding box.

## Results

### [ENA24](http://lila.science/datasets/ena24detection)

```
Mean: rgb(93, 96, 79)
Mean: #5d604f
```

Complimentary color [(calc)](https://www.sessions.edu/color-calculator/): `#604f5d`

## Installation

```bash
$ cargo install git+https://github.com/bencevans/detection-color
```

### Usage


```bash
$ detection-color --help
Calculate the mean pixels of 1px inset and outset boxes around each object in a COCO dataset

Usage: detection-color <IMAGE_DIR> <COCO_PATH>

Arguments:
  <IMAGE_DIR>  Path to the image_dir
  <COCO_PATH>  Path to COCO annotations

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## License

MIT