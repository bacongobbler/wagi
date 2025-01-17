fn main() {
  let url = "https://api.brigade.sh/healthz";

  match get_body(url) {
    Ok((status_code, _)) => {
      println!("Content-Type: text/plain\n");
      if status_code == 200 {
          println!("api.brigade.sh is HEALTHY");
      } else {
          println!("api.brigade.sh is UNHEALTHY");
      }
    },
    Err(e) => {
      println!("Status: 500");
      println!("Content-Type: text/plain\n");
      println!("Failed to make request: {}", e);
    }
  }
}

fn get_body(url: &str) -> anyhow::Result<(http::StatusCode, String)> {
  let req = http::request::Builder::new().uri(url).body(None)?;
  let mut res = wasi_experimental_http::request(req)?;

  let body = res.body_read_all()?;
  let body_text = std::str::from_utf8(&body)?;

  Ok((res.status_code, body_text.to_owned()))
}
