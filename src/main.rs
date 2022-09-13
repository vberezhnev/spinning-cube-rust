static mut A: f32 = 0.0;
static mut B: f32 = 0.0;
static mut C: f32 = 0.0;

// static mut width: i32 = 160;
// static mut height: i32 = 44;
// static mut backgroundASCIICode: char = '.';
// static mut horizontalOffset: f32;
// static mut K1: i32 = 40;

// static mut incrementSpeed: f32 = 0.6;

// static mut x: f32;
// static mut y: f32;
// static mut z: f32;

// static mut ooz: f32;
// static mut xp: int32;
// static mut yp: int32;
// static mut idx: int32;

fn calculateX(i: f32, j: f32, k: f32) -> f32 {
    return j * A.sin() * B.sin() * C.cos() - k * A.cos() * B.sin() * C.cos()
        + j * A.cos() * C.sin()
        + k * A.sin() * C.sin()
        + i * B.sin() * C.cos();
}

fn calculateY(i: f32, j: f32, k: f32) -> f32 {
    return i * A.cos() * C.cos() + k * A.sin() * C.cos() - j * A.sin() * B.sin() * C.sin()
        + k * A.cos() * B.sin() * C.sin()
        - i * B.cos() * C.sin();
}

fn calculateZ(i: f32, j: f32, k: f32) -> f32 {
    return k * A.cos() * B.cos() - j * A.sin() * B.cos() + i * B.sin();
}

fn calculateForSurface(cubeX: f32, cubeY: f32, cubeZ: f32, ch: f32) {
    let width: f32 = 160.0;
    let height: f32 = 44.0;
    let mut zBuffer: [i32; 160 * 44];
    let mut buffer: [i32; 160 * 44];
    let K1: f32 = 40.0;

    let distanceFromCam: f32 = 100.0;
    let horizontalOffset;

    let x: f32 = calculateX(cubeX, cubeY, cubeZ);
    let y: f32 = calculateY(cubeX, cubeY, cubeZ);
    let z: f32 = calculateZ(cubeX, cubeY, cubeZ) + distanceFromCam;

    let ooz: f32 = 1.0 / z;

    let xp: f32 = width / 2.0 + horizontalOffset + K1 * ooz * x * 2.0;
    let yp: f32 = height / 2.0 + K1 * ooz * y;

    let idx: f32 = xp + yp * width;
    if idx >= 0.0 && idx < width * height {
        if ooz > zBuffer[idx] {
            zBuffer[idx] = ooz;
            buffer[idx] = ch;
        }
    }
}

fn main() {
    println!("по идее все ок");
}
