trait OnWeatherDataUpdate {
    fn update(&self, weather_data: &WeatherData);
    fn get_id(&self) -> i32;
}

trait IWeatherData<'a> {
    fn register_display(&mut self, observer: &'a dyn OnWeatherDataUpdate);
    fn unregister_display(&mut self, observer: &'a dyn OnWeatherDataUpdate);
    fn notify_displays(&self);
    fn set_humidity(&mut self, n:i32);
    fn set_temp(&mut self, n:i32);
    fn set_pressure(&mut self, n:i32);
}

struct WeatherData<'a> {
    temp: i32,
    humidity: i32,
    pressure: i32,
    displays: Vec<&'a dyn OnWeatherDataUpdate>,
}
impl<'a> WeatherData<'a> {
    fn new() -> WeatherData<'a> {
        WeatherData {
            temp: 0,
            humidity: 0,
            pressure: 0,
            displays: Vec::new(),
        }
    }
}

impl<'a> IWeatherData<'a> for WeatherData<'a> {
    fn register_display(&mut self, display: &'a dyn OnWeatherDataUpdate) {
        println!("registering id:{}", display.get_id());
        self.displays.push(display);
    }
    fn unregister_display(&mut self, display: &'a dyn OnWeatherDataUpdate) {
        println!("unregister id:{}", display.get_id());
        if let Some(idx) = self.displays.iter().position(|x| x.get_id() == display.get_id()) {
            self.displays.remove(idx);
        }
    }
    fn notify_displays(&self) {
        for item in self.displays.iter() {
            item.update(&self);
        }
    }
    fn set_humidity(&mut self, n:i32) {
        println!("setting humidity");
        self.humidity = n;
        self.notify_displays();
    }
    fn set_temp(&mut self, n:i32) {
        println!("setting temp");
        self.temp = n;
        self.notify_displays();
    }
    fn set_pressure(&mut self, n:i32) {
        println!("setting pressure");
        self.pressure = n;
        self.notify_displays();
    }
}

#[derive(PartialEq)]
struct CurrentConditionDisplay {
    id: i32,
}
impl OnWeatherDataUpdate for CurrentConditionDisplay {
    fn update(&self, weather_data: &WeatherData) {
        println!("CurrentConditionDisplay id:{} temp:{} humidity:{} atm:{}", self.id, weather_data.temp, weather_data.humidity, weather_data.pressure);
    }
    fn get_id(&self) -> i32 {
        self.id
    }
}

struct StatisticsDisplay {
    id: i32
}
impl OnWeatherDataUpdate for StatisticsDisplay {
    fn update(&self, weather_data: &WeatherData) {
        println!("StatisticsDisplay: temp:{} hum:{} atm:{}", weather_data.temp, weather_data.humidity, weather_data.pressure);
    }
    fn get_id(&self) -> i32 {
        self.id
    }
}

/**
 * notes:
 *  - need to better understand lifetimes
 *  - https://doc.rust-lang.org/rust-by-example/scope/lifetime.html
 */
pub fn run() {
    let mut subject = WeatherData::new();
    let observer_a = CurrentConditionDisplay { id: 1 };
    let observer_b = StatisticsDisplay { id: 2 };

    subject.register_display(&observer_a);
    subject.register_display(&observer_b);
    subject.set_humidity(32);
    subject.set_temp(27);

    subject.unregister_display(&observer_b);
    subject.set_pressure(41);
}