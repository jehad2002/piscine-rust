#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String, // اسم الضوء
    pub brightness: u8, // سطوع الضوء
}

impl Light {
    // دالة لإنشاء ضوء جديد بالاسم alias وسطوع 0
    pub fn new(alias: &str) -> Self {
        Light {
            alias: alias.to_string(),
            brightness: 0,
        }
    }
}

// دالة لتغيير سطوع الضوء حسب الاسم alias
pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for light in lights.iter_mut() {
        if light.alias == alias {
            light.brightness = value;
            break; // نوقف بعد ما نلقى الضوء المناسب
        }
    }
}
