# Rayo

A ray tracer implementation written in rust. Inspired by the amazing book: [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

# Spotlight

Here is a gallery of some of the best renders:

![Best render](renders/rednfuzzy.png)

# Usage

Rayo is (currenty) a command line tool. The available flags and options are:

```
USAGE:
    rayo [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --aspect <ASPECT-RATIO>        Aspect ratio [default: 16/9]
    -d, --depth <MAX-DEPTH>            Maximum recursion depth [default: 30]
    -n, --num-samples <NUM-SAMPLES>    Number of samples per pixel [default: 100]
    -o, --out <FILE>                   Rendered image path [default: render.png]
    -r, --resolution <RESOLUTION>      Horizontal image resolution [default: 480]
```

# Features

Many features are yet to be implemented

- [x] Nice CLI
- [ ] Reading scenes from json or some other similar format
- [ ] Support for OBJ files for meshes

## Shapes

- [x] Spheres
- [ ] Axis aligned boxes
- [ ] Triangle meshes

## Materials

- [x] Lambertian diffuse materials
- [x] Metalic materials
- [ ] Glass and Dielectrics

## Effects

- [x] Anti aliasing
- [ ] Depth of field
- [ ] Motion blur 
- [ ] Adjustable camera positions

## Efficiency enhancements

- [x] parallelize on CPU
- [ ] parallelize on GPU
- [ ] bounding volumes

## Possible things but unlikely

- make an opengl viewer which allows you to set up scene and then switch to raytraced mode
- compile this to webgl and wasm and run in browser?

