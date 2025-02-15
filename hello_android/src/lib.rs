#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod hello_android {
    extern crate jni;

    use jni::JNIEnv;
    use self::jni::objects::{JClass, JObject, JString};
    use self::jni::sys::{jstring};

    use candle_core::{DType, Device, Tensor};

    #[no_mangle]
    pub unsafe extern "C" fn Java_ai_baseweight_hellobasicrust_MainActivity_stringFromRust(
        env: JNIEnv,
        _obj: JObject,
    ) -> jstring {
        let output = env.new_string("Hello, from Rust!").expect("Failed to create Java string");
        return **output 
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_ai_baseweight_hellobasicrust_MainActivity_tensorFromRust(
        env: JNIEnv,
        _obj: JObject,
    ) -> jstring {
        let device = Device::Cpu;
        let dtype = DType::F32;
        let shape = &[2, 3];
        let tensor = Tensor::randn(0f32, 1., shape, &device).unwrap();
        let output = env.new_string(format!("{}", tensor)).expect("Failed to create Java string");
        return **output 
    } 
}