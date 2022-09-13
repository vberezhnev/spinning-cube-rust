static A: f32 = None;
static B: f32 = None;
static C: f32 = None;

// static mut width: i32 = 160;
// static mut height: i32 = 44;
// static mut backgroundASCIICode: char = '.';
// static mut horizontalOffset: f32;
// static mut K1: i32 = 40;

// static mut incrementSpeed: f32 = 0.6;

static mut x: f32;
static mut y: f32;
static mut z: f32;

static mut ooz: f32;
static mut xp: int32;
static mut yp: int32;
static mut idx: int32;

fn calculateX(i: i32, j: i32, k: i32) {
    return f32::from(j) * A.sin() * B.sin() * C.cos() - f32::from(k) * A.cos() * B.sin() * C.cos()
        + f32::from(j) * A.cos() * C.sin()
        + f32::from(k) * A.sin() * C.sin()
        + f32::from(i) * B.sin() * C.cos();
}

fn calculateY(i: i32, j: i32, k: i32) {
    return f32::from(i) * A.cos() * C.cos() + f32::from(k) * A.sin() * C.cos()
        - f32::from(j) * A.sin() * B.sin() * C.sin()
        + f32::from(k) * A.cos() * B.sin() * C.sin()
        - f32::from(i) * B.cos() * C.sin();
}

fn calculateZ(i: int32, j: int32, k: int32) {
    return f32::from(k) * A.cos() * B.cos() - f32::from(j) * A.sin() * B.cos()
        + f32::from(i) * B.sin();
}

fn calculateForSurface(cubeX: i32, cubeY: i32, cubeZ: i32, ch: f32) {
    x = calculateX(cubeX, cubeY, cubeZ);
    y = calculateY(cubeX, cubeY, cubeZ);
    z = calculateZ(cubeX, cubeY, cubeZ) + distanceFromCam;

    ooz = 1 / z;

    xp = (int)(width / 2 + horizontalOffset + K1 * ooz * x * 2);
    yp = (int)(height / 2 + K1 * ooz * y);

    idx = xp + yp * width;
    if (idx >= 0 && idx < width * height) {
        if (ooz > zBuffer[idx]) {
            zBuffer[idx] = ooz;
            buffer[idx] = ch;
        }
    }
}

fn main() {
    println!("по идее все ок");
}
