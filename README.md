# toy-raytracer-rust

> 用 Rust 抄 _Ray Tracing in a Weekend_

## 使用方法

```sh
git clone https://github.com/oopsno/toy-raytracer-rust.git
cd toy-raytracer-rust
cargo run --release
```

## 命令行接口

```
toy-raytracer-rust 0.1

USAGE:
    toy-raytracer-rust [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --aspect-ratio <aspect-ratio>               [default: 1.5]
    -i, --image-width <image-width>                 [default: 1200]
    -m, --max-depth <max-depth>                     [default: 50]
    -n, --num-threads <num-threads>                 [default: 0]
    -o, --output <output>                           [default: output.png]
    -s, --samples-per-pixel <samples-per-pixel>     [default: 500]
    -s, --scene <scene>                             [default: weekend]
```

scene 取值可为以下五项之一
+ `diffuse-spheres` Image 10: 带有半球散射的漫反射球体
+ `shiny-metal` Image 11: 闪亮金属
+ `fuzzy-metal` Image 12: 模糊金属
+ `hollow-glass-spheres` Image 16: 中空玻璃球
+ `weekend` 最终场景

## 预览

以默认参数渲染的上述场景位于 `artifacts` 目录下。

### 最终场景 

![](https://github.com/oopsno/toy-raytracer-rust/blob/master/artifacts/weekend.png)

### Image 10: 带有半球散射的漫反射球体

![](https://github.com/oopsno/toy-raytracer-rust/blob/master/artifacts/diffuse-spheres.png)

### Image 11: 闪亮金属

![](https://github.com/oopsno/toy-raytracer-rust/blob/master/artifacts/shiny-metal.png)

### Image 12: 模糊金属

![](https://github.com/oopsno/toy-raytracer-rust/blob/master/artifacts/fuzzy-metal.png)

### Image 16: 中空玻璃球

![](https://github.com/oopsno/toy-raytracer-rust/blob/master/artifacts/hollow-glass-spheres.png)
