use mysql::prelude::*;
use mysql::*;
use std::error::Error;

pub fn connectdb(
    un: Option<String>,
    pw: Option<String>,
    burl: Option<String>,
) -> Result<Pool, Box<dyn Error>> {
    let username = un.unwrap_or_else(|| {
        dotenvy::dotenv().ok();
        dotenvy::var("DCIMUSER").expect("No Username via env")
    });
    let password = pw.unwrap_or_else(|| {
        dotenvy::dotenv().ok();
        dotenvy::var("DCIMPASSWORD").expect("No Password via env")
    });
    let baseurl = burl.unwrap_or_else(|| {
        dotenvy::dotenv().ok();
        dotenvy::var("DCIMHOST").expect("No BASEURL via env")
    });

    let url = format!("mysql://{}:{}@{}:3306/dcim", username, password, baseurl);
    let opts = Opts::from_url(url.as_str())?;
    let pool = Pool::new(opts)?;
    Ok(pool)
}

pub fn queryall(pool: Pool) -> Result<Vec<String>, Box<dyn Error>> {
    let mut conn = pool.get_conn()?;
    let allports = conn.query_map(
        r#"
        SELECT Notes
        FROM fac_Ports
        WHERE (Notes REGEXP '^[0-9]+.[EU0-9]+.[0-9]+-[0-9a-z/,]+?$'
    OR Notes REGEXP '^MT-|.*APD.*|.*APP.*|.*APR.*|.*APM.*|.*APK.*')
    AND PortNumber < 0
    "#,
        |notes: String| notes,
    )?;
    Ok(allports)
}

pub fn myquery(pool: Pool, notes: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut conn = pool.get_conn()?;
    let selected: Vec<Vec<String>> = conn.exec_map(
        r#"
        SELECT p1.PortNumber, p1.DeviceID,
        p2.ConnectedDeviceID,
        p2.ConnectedPort, d1.DeviceID,
        d1.Label, p3.Notes,
        p3.Label, d1.PrimaryIP
        FROM fac_Ports p1
        JOIN fac_Ports p2 ON p1.DeviceID = p2.DeviceID AND p1.PortNumber = -p2.PortNumber
        JOIN fac_Device d1 ON d1.DeviceID = p2.ConnectedDeviceID
        JOIN fac_Ports p3 ON p3.DeviceID = p2.ConnectedDeviceID AND p3.PortNumber = p2.ConnectedPort
        WHERE p1.Notes = ? LIMIT 1
        "#,
        (notes.trim(),),
        |(
            _port_number,
            _device_id,
            _connected_device_id,
            _connected_port,
            switch_device_id,
            switch_label,
            port_notes,
            switch_port,
            switch_ip,
        ): (i32, i32, i32, i32, i32, String, String, String, String)| {
            vec![
                switch_label,
                port_notes,
                switch_port,
                switch_ip,
                switch_device_id.to_string(),
            ]
        },
    )?;
    if let Some(row) = selected.into_iter().next() {
        Ok(row)
    } else {
        Err("Row not found".into())
    }
}
