use core::str;
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{modem::Modem, peripheral::Peripheral},
    nvs::{EspNvsPartition, NvsDefault},
    ping::{self, EspPing},
    sys::EspError,
    timer::{EspTimerService, Task},
    wifi::{AsyncWifi, AuthMethod, ClientConfiguration, Configuration, EspWifi},
};
use log::info;

const WIFI_SSID: &str = env!("RUST_ESP32_WIFI_SSID");
const WIFI_PASSWORD: &str = env!("RUST_ESP32_WIFI_PASSWORD");

fn str_to_string32(s: &str) -> heapless::String<32> {
    let mut string = heapless::String::<32>::new();
    string.push_str(s).expect("Failed to push string");
    string
}

fn str_to_string64(s: &str) -> heapless::String<64> {
    let mut string = heapless::String::<64>::new();
    string.push_str(s).expect("Failed to push string");
    string
}

pub fn init_wifi(
    modem: impl Peripheral<P = Modem> + 'static,
    sysloop: EspSystemEventLoop,
    nvs: Option<EspNvsPartition<NvsDefault>>,
    timer_service: EspTimerService<Task>,
) -> Result<AsyncWifi<EspWifi<'static>>, EspError> {
    use futures::executor::block_on;
    let mut wifi = AsyncWifi::wrap(
        EspWifi::new(modem, sysloop.clone(), nvs)?,
        sysloop,
        timer_service.clone(),
    )?;

    block_on(connect_wifi(&mut wifi))?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    println!("Wifi DHCP info: {:?}", ip_info);

    EspPing::default().ping(ip_info.subnet.gateway, &ping::Configuration::default())?;
    Ok(wifi)
}

async fn connect_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> Result<(), EspError> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: str_to_string32(WIFI_SSID),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: str_to_string64(WIFI_PASSWORD),
        channel: None,
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start().await?;
    info!("Wifi started");

    wifi.connect().await?;
    info!("Wifi connected");

    wifi.wait_netif_up().await?;
    info!("Wifi netif up");

    Ok(())
}
