use std::thread::sleep;
use std::time::Duration;

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals;
use esp_idf_svc::hal as esp_idf_hal;
use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::*;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::ClientConfiguration;
use esp_idf_svc::wifi::Configuration;
use esp_idf_svc::wifi::EspWifi;
use heapless::String;


const SSID: &str = env!("WIFI_SSID");
const PASS: &str = env!("WIFI_PASS");
fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_hal::sys::link_patches();
    log::info!("DEVICE ON");
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    // let tx = peripherals.pins.gpio12;
    // let rx = peripherals.pins.gpio13;
    let mut wifi_driver = EspWifi::new(
        peripherals.modem,
        sys_loop,
        Some(nvs)
    )?;
    let mut ssid: String<32> = String::try_from(SSID).unwrap();
    let mut passwd: String<64> = String::try_from(PASS).unwrap();
    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration{
        ssid: ssid,
        password: passwd,
        ..Default::default()
    }))?;
    wifi_driver.start()?;
    wifi_driver.connect()?;
    while !wifi_driver.is_connected()?{
        let config = wifi_driver.get_configuration()?;
        println!("Waiting for station {:?}", config);
    }
    log::info!("Should be connected now");
    loop{
        log::info!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info()?);
        // println!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info().unwrap());
        sleep(Duration::new(10,0));
        
    }
}
