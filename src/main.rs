use sysinfo::{System, SystemExt};

const RESET: &str = r#"\x1B[0m"#;
const BLACK: &str = r#"\x1B[30m"#; /* Black */
const RED: &str = r#"\x1B[31m"#; /* Red */
const GREEN: &str = r#"\x1B[32m"#; /* Green */
const YELLOW: &str = r#"\x1B[33m"#; /* Yellow */
const BLUE: &str = r#"\x1B[34m"#; /* Blue */
const MAGENTA: &str = r#"\x1B[35m"#; /* Magenta */
const CYAN: &str = r#"\x1B[36m"#; /* Cyan */
const WHITE: &str = r#"\x1B[37m"#; /* White */

fn main() {
    get_ascci();
}

fn format_ascci(ascci: String) -> String {
    let sys = System::new_all();
    let ascci_replaced = ascci
        .replace("OS_NAME", sys.name().unwrap().as_str())
        .replace("KERNEL_VERSION", sys.kernel_version().unwrap().as_str())
        .replace("UPTIME", sys.uptime().to_string().as_str())
        .replace("USED_MEMORY", sys.used_memory().to_string().as_str())
        .replace("TOTAL_MEMORY", sys.total_memory().to_string().as_str())
        .replace("USED_SWAP", sys.used_swap().to_string().as_str())
        .replace("TOTAL_SWAP", sys.total_swap().to_string().as_str())
        .replace("RESET", RESET)
        .replace("BLACK", BLACK)
        .replace("RED", RED)
        .replace("GREEN", GREEN)
        .replace("YELLOW", YELLOW)
        .replace("BLUE", BLUE)
        .replace("MAGENTA", MAGENTA)
        .replace("CYAN", CYAN)
        .replace("WHITE", WHITE);

    return ascci_replaced;
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn get_ascci() -> Result<(), reqwest::Error> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let url = "https://raw.githubusercontent.com/TheHolyTachanka/rufetch/main/ascii/".to_owned()
        + &sys.name().unwrap().to_owned();

    let res = reqwest::get(&url).await?;

    if res.status().is_success() {
        let ascci = res.text().await?;

        let better_ascci = format_ascci(ascci);

        println!("{}", better_ascci.to_owned());
    } else {
        let res = reqwest::get(
            "https://raw.githubusercontent.com/TheHolyTachanka/rufetch/main/ascii/default",
        )
        .await?;

        let ascci = res.text().await?;

        let better_ascci = format_ascci(ascci);

        println!("{}", better_ascci);
    }

    Ok(())
}
