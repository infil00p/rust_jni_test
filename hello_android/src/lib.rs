#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod hello_android {
    extern crate jni;

    use jni::JNIEnv;
    use self::jni::objects::{JClass, JObject, JString};
    use self::jni::sys::{jstring};

    use candle_core::{DType, Device, Tensor, Error};
    use candle_nn::{Linear, Module};

    struct Model {
        first: Linear,
        second: Linear,
    }

    impl Model {
        fn forward(&self, image: &Tensor) -> Result<Tensor, Error> {
            let x = self.first.forward(image)?;
            let x = x.relu()?;
            self.second.forward(&x)
        }
    }

    fn mnist() -> Result<Tensor, Error> {
        // We don't have CUDA on an Android device
        let device = Device::Cpu;

        // This has changed (784, 100) -> (100, 784) !
        let weight = Tensor::randn(0f32, 1.0, (100, 784), &device)?;
        let bias = Tensor::randn(0f32, 1.0, (100, ), &device)?;
        let first = Linear::new(weight, Some(bias));
        let weight = Tensor::randn(0f32, 1.0, (10, 100), &device)?;
        let bias = Tensor::randn(0f32, 1.0, (10, ), &device)?;
        let second = Linear::new(weight, Some(bias));
        let model = Model { first, second };

        // TODO: Use JNI to grab the contents of an image and feed it into
        // the tensor object here
        let dummy_image = Tensor::randn(0f32, 1.0, (1, 784), &device)?;
        model.forward(&dummy_image)
    }

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
        let tensor = mnist().unwrap_or_else(|e| {
            // Handle the error case by returning a dummy tensor
            // In practice, you might want better error handling
            Tensor::new(&[0.0f32], &Device::Cpu).unwrap()
        });
        
        let output = env.new_string(format!("{:?}", tensor))
            .expect("Failed to create Java string");
        return **output 
    } 
}
